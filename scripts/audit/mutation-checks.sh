#!/bin/bash
# scripts/audit/mutation-checks.sh
# Run mutation checks for mutation-resistant tests.
# OPERATOR-RUN ONLY. Executor must not run this script.

set -euo pipefail

MUTATIONS_JSON="scripts/audit/mutations.json"
RESULTS_JSON="mutation-checks-results.json"

if [ ! -f "$MUTATIONS_JSON" ]; then
  echo "FAIL: mutations.json missing"
  exit 1
fi

PY_CMD="python3"
if ! command -v python3 >/dev/null 2>&1; then
  PY_CMD="python"
fi

$PY_CMD -c '
import json
import shutil
import subprocess
import sys
import time
import os

def mutate_file(filepath, func_name, language):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    
    idx = content.find(func_name)
    if idx == -1:
        raise ValueError(f"Function {func_name} not found in {filepath}")
    
    brace_idx = content.find("{", idx)
    if brace_idx == -1:
        raise ValueError(f"Opening brace not found after {func_name}")
    
    count = 1
    curr = brace_idx + 1
    while count > 0 and curr < len(content):
        if content[curr] == "{":
            count += 1
        elif content[curr] == "}":
            count -= 1
        curr += 1
    
    if count > 0:
        raise ValueError(f"Matching closing brace not found for {func_name}")
    
    body = " unimplemented!(); " if language == "rust" else " throw new Error(\"mutation\"); "
    new_content = content[:brace_idx + 1] + body + content[curr - 1:]
    
    # Backup original
    shutil.copyfile(filepath, filepath + ".bak")
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(new_content)

def restore_file(filepath):
    bak = filepath + ".bak"
    if os.path.exists(bak):
        shutil.copyfile(bak, filepath)
        os.remove(bak)

def run_test(test_name):
    if "vitest" in test_name or "useapp" in test_name:
        cmd = f"npx vitest run {test_name}"
    else:
        cmd = f"cd src-tauri && cargo test {test_name}"
    
    print(f"Running command: {cmd}")
    res = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return res.returncode == 0

# Main execution
with open("scripts/audit/mutations.json", "r", encoding="utf-8") as f:
    mutations = json.load(f)

results = []
host_platform = sys.platform
platform_map = {"win32": "windows", "linux": "linux", "darwin": "darwin"}
mapped_host = platform_map.get(host_platform, host_platform)

for m in mutations:
    test_name = m["test"]
    filepath = m["file"]
    func_name = m["function"]
    lang = m["language"]
    platforms = m.get("platforms")
    
    print(f"\n--- Testing mutation for: {test_name} ---")
    
    # Check platform limitation
    if platforms and mapped_host not in platforms:
        print(f"Skipping {test_name}: host platform {mapped_host} not in supported platforms {platforms}")
        results.append({
            "test": test_name,
            "platform_skipped": mapped_host,
            "reason": "not in platforms array"
        })
        continue
    
    # Check if file exists
    if not os.path.exists(filepath):
        print(f"Skipping {test_name}: file {filepath} not found on this host")
        continue
        
    try:
        # Mutate
        mutate_file(filepath, func_name, lang)
        
        # Run test - expect it to FAIL when mutated
        passed_under_mutation = run_test(test_name)
        
        # Restore
        restore_file(filepath)
        
        # Mutation is caught if the test fails under mutation
        mutation_caught = not passed_under_mutation
        print(f"Result: mutation_caught={mutation_caught}")
        
        results.append({
            "test": test_name,
            "function": f"{filepath}::{func_name}",
            "mutation_caught": mutation_caught,
            "platform_tested": [mapped_host]
        })
        
    except Exception as e:
        print(f"Error executing mutation: {e}")
        restore_file(filepath)
        results.append({
            "test": test_name,
            "function": f"{filepath}::{func_name}",
            "mutation_caught": False,
            "platform_tested": [mapped_host],
            "error": str(e)
        })

output = {
    "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    "host_platform": host_platform,
    "results": results
}

with open("mutation-checks-results.json", "w", encoding="utf-8") as f:
    json.dump(output, f, indent=2)

print("\nMutation checks complete. Results written to mutation-checks-results.json")
'
