import os
import json
import hashlib
import subprocess

def _compute_sha256(filepath):
    sha256_hash = hashlib.sha256()
    with open(filepath, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

def audit_team_zero_blockers(verdict_path=".agent-runs/2026-05-26-civicnewspaper-v020-ship/audit-team-verdict.json", expected_sha=""):
    if not os.path.exists(verdict_path):
        raise Exception(f"Verdict file missing at {verdict_path}")
    
    file_sha = _compute_sha256(verdict_path)
    if file_sha != expected_sha:
        raise Exception(f"Verdict file hash mismatch. Expected: {expected_sha}, Got: {file_sha}")
        
    with open(verdict_path, "r", encoding="utf-8") as f:
        data = json.load(f)
        
    required = {
        "blockers", "criticals", "verdict", "audit_artifact", "auditor",
        "mutation_checks_results_sha256", "clippy_platforms_run", "mutation_checks_platforms"
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
