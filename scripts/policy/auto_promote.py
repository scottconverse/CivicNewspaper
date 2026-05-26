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
        
    blockers = data.get("blockers", 0)
    criticals = data.get("criticals", 0)
    
    if blockers == 0 and criticals == 0:
        return "PASS"
    else:
        return "FAIL"

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
