import os
import json
import hashlib
import subprocess
import urllib.parse
import re
import glob

def _is_valid_url(url):
    # Structural validation only — deliberately NO network I/O (ENG-012). This
    # runs inside the citation-policy gate in CI, where a live HEAD/GET would be
    # non-deterministic (a flaky or offline endpoint would fail an otherwise
    # valid citation), slow (two 5s timeouts per URL), and an SSRF surface
    # (following attacker-controlled URLs from promoted text). We only assert the
    # citation is a well-formed http(s) URL that points at a specific resource.
    try:
        parsed = urllib.parse.urlparse(url)
        if parsed.scheme not in ("http", "https"):
            return False
        if not parsed.netloc:
            return False
        # GENERIC_URL check: reject a bare host without a path (WP-1) — a
        # citation must point at a specific page, not just a domain root.
        if not parsed.path.strip("/"):
            return False
        return True
    except Exception:
        return False

def _is_valid_sha(sha):
    # Check if git SHA is valid
    try:
        # Run git rev-parse --quiet --verify
        res = subprocess.run(
            ["git", "rev-parse", "--quiet", "--verify", sha],
            capture_output=True, text=True
        )
        if res.returncode == 0:
            return True
    except Exception:
        pass
    return False

def _check_citation_in_window(lines, start_idx):
    # Narrative phrase is on line start_idx
    # Window is start_idx to start_idx + 3
    end_idx = min(start_idx + 3, len(lines) - 1)
    window_lines = lines[start_idx:end_idx + 1]
    
    # 1. Look for URL
    url_pattern = re.compile(r"https?://[^\s)\]]+")
    for line in window_lines:
        urls = url_pattern.findall(line)
        for url in urls:
            if _is_valid_url(url):
                return True
                
    # 2. Look for SHA256 or Git SHA
    hex_pattern = re.compile(r"\b[a-fA-F0-9]{7,64}\b")
    for line in window_lines:
        hexes = hex_pattern.findall(line)
        for h in hexes:
            if len(h) == 64:
                # Valid SHA256 reference
                return True
            elif 7 <= len(h) <= 40:
                # Check if it is a valid Git commit SHA
                if _is_valid_sha(h):
                    return True
                    
    # 3. Look for build-log filename reference
    build_log_pattern = re.compile(r"\bbuild[-_]log\.txt\b|\bbuild[-_]log\b", re.IGNORECASE)
    for line in window_lines:
        if build_log_pattern.search(line):
            return True
            
    return False

def _compute_sha256(filepath):
    sha256_hash = hashlib.sha256()
    with open(filepath, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

def audit_team_zero_blockers(verdict_path, expected_sha=""):
    if not os.path.exists(verdict_path):
        raise Exception(f"Verdict file missing at {verdict_path}")
    
    file_sha = _compute_sha256(verdict_path)
    if file_sha != expected_sha:
        raise Exception(f"Verdict file hash mismatch. Expected: {expected_sha}, Got: {file_sha}")
        
    with open(verdict_path, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    required = {
        "blockers", "criticals", "verdict", "audit_artifact", "auditor",
        "mutation_checks_results_sha256", "clippy_platforms_run", "mutation_checks_platforms",
        "checkpoint_audits_clean"
    }
    missing = required - set(data.keys())
    if missing:
        raise ValueError(f"verdict file missing required fields: {missing}")
        
    if type(data["blockers"]) is not int:
        raise ValueError("blockers must be an integer")
    if type(data["criticals"]) is not int:
        raise ValueError("criticals must be an integer")
        
    if data["verdict"] != "PROMOTE":
        raise ValueError("verdict must be PROMOTE")
        
    if data["checkpoint_audits_clean"] is not True:
        raise ValueError("checkpoint_audits_clean must be True")
        
    if not isinstance(data["mutation_checks_results_sha256"], str) or not data["mutation_checks_results_sha256"]:
        raise ValueError("mutation_checks_results_sha256 must be a non-empty string")
        
    if not isinstance(data["clippy_platforms_run"], list) or len(data["clippy_platforms_run"]) == 0:
        raise ValueError("clippy_platforms_run must be a non-empty list")
        
    # Read mutations.json and verify platform intersections
    mutations_path = "scripts/audit/mutations.json"
    if os.path.exists(mutations_path):
        with open(mutations_path, "r", encoding="utf-8") as f:
            mutations = json.load(f)
        
        verdict_platforms = set(data["mutation_checks_platforms"])
        for entry in mutations:
            entry_platforms = entry.get("platforms")
            if entry_platforms:
                intersection = verdict_platforms.intersection(set(entry_platforms))
                if not intersection:
                    raise ValueError(f"No platform intersection for mutation test: {entry['test']}")

    # Checkpoint Audits validation (WI-1)
    run_dir = os.path.dirname(verdict_path)
    checkpoint_shas_path = os.path.join(run_dir, "audits", "checkpoint-shas.txt")
    if not os.path.exists(checkpoint_shas_path):
        raise ValueError(f"checkpoint-shas.txt missing at {checkpoint_shas_path}")
        
    import re
    with open(checkpoint_shas_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
        
    for line in lines:
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split(None, 1)
        if len(parts) < 2:
            continue
        pinned_sha, filename = parts[0], parts[1]
        
        # Verify file exists
        if not os.path.exists(filename):
            raise Exception(f"Checkpoint file missing: {filename}")
            
        # Verify SHA
        computed_sha = _compute_sha256(filename)
        if computed_sha != pinned_sha:
            raise Exception(f"Checkpoint file hash mismatch for {filename}. Expected: {pinned_sha}, Got: {computed_sha}")
            
        # Parse severity rollup
        with open(filename, "r", encoding="utf-8") as cf:
            content = cf.read()
            
        rollup_idx = content.lower().find("severity rollup")
        if rollup_idx != -1:
            rollup_section = content[rollup_idx:rollup_idx+500]
        else:
            rollup_section = content
            
        blockers_count = None
        criticals_count = None
        
        # Regex anchored per Wnit-5
        pattern = re.compile(r"^- (Blocker|Critical|Major|Minor|Nit):\s*(\d+)\s*$", re.IGNORECASE)
        for line in rollup_section.splitlines():
            m = pattern.match(line)
            if m:
                severity = m.group(1).lower()
                count = int(m.group(2))
                if severity == "blocker":
                    blockers_count = count
                elif severity == "critical":
                    criticals_count = count

        if blockers_count is None:
            raise ValueError(f"No blocker count found in {filename}")
        if criticals_count is None:
            raise ValueError(f"No critical count found in {filename}")
            
        if blockers_count > 0 or criticals_count > 0:
            raise ValueError(f"Non-zero blockers or criticals count in {filename}")

    # Enforce §0.17 narrative citation rules (WP-1)
    stage_reports = glob.glob(os.path.join(run_dir, "stage-*-report.md"))
    narratives = [
        "by design",
        "is normal",
        "expected behavior",
        "this is correct because"
    ]
    for report_path in stage_reports:
        with open(report_path, "r", encoding="utf-8") as rf:
            report_lines = rf.readlines()
        for i, line in enumerate(report_lines):
            has_narrative = False
            for phrase in narratives:
                if phrase in line.lower():
                    has_narrative = True
                    break
            if has_narrative:
                if not _check_citation_in_window(report_lines, i):
                    raise ValueError(
                        f"Narrative phrase found in {os.path.basename(report_path)} "
                        f"on line {i+1} lacks valid cited evidence in 3-line window."
                    )

    blockers = data["blockers"]
    criticals = data["criticals"]
    
    if blockers == 0 and criticals == 0:
        return "PASS"
    else:
        raise ValueError("Non-zero blockers or criticals count")

def release_artifacts_exist_on_github(tag="v0.2.0"):
    try:
        cmd = f"gh release view {tag} --json assets --jq \".assets | length\""
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, check=True)
        length_str = result.stdout.strip()
        if not length_str:
            return "FAIL"
        length = int(length_str)
        if length >= 3:
            return "PASS"
        else:
            return "FAIL"
    except Exception as e:
        raise Exception(f"Failed to check release artifacts on GitHub: {e}")
