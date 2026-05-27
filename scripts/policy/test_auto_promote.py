import json
import hashlib
import tempfile
import os
import pytest
from auto_promote import audit_team_zero_blockers

def test_empty_file():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        f.write('{}')
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        with pytest.raises(ValueError):
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    finally:
        os.remove(p)

def test_missing_blockers():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        with pytest.raises(ValueError):
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    finally:
        os.remove(p)

def test_blockers_one():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': 1, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        with pytest.raises(ValueError):
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    finally:
        os.remove(p)

def test_criticals_one():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': 0, 'criticals': 1, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        with pytest.raises(ValueError):
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    finally:
        os.remove(p)

def test_verdict_block():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': 0, 'criticals': 0, 'verdict': 'BLOCK', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        with pytest.raises(ValueError):
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    finally:
        os.remove(p)

def test_wrong_sha():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    try:
        with pytest.raises(Exception) as excinfo:
            audit_team_zero_blockers(verdict_path=p, expected_sha="wrong-sha")
        assert "hash mismatch" in str(excinfo.value)
    finally:
        os.remove(p)

def test_blockers_string():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': "0", 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        with pytest.raises(ValueError) as excinfo:
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
        assert "blockers must be an integer" in str(excinfo.value)
    finally:
        os.remove(p)

def test_blockers_null():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': None, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        with pytest.raises(ValueError) as excinfo:
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
        assert "blockers must be an integer" in str(excinfo.value)
    finally:
        os.remove(p)

def test_passes_correct():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    try:
        res = audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
        assert res == "PASS"
    finally:
        os.remove(p)

def test_mutation_platforms_intersection_failure():
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump({
            'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
            'mutation_checks_results_sha256': 'some-sha',
            'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['windows']  # Only windows, should fail since mutations require linux/darwin
        }, f)
        p = f.name
    sha = hashlib.sha256(open(p, 'rb').read()).hexdigest()
    
    # Temporarily write a mock mutations.json to check the logic
    mock_mutations = "scripts/audit/mutations.json"
    os.makedirs(os.path.dirname(mock_mutations), exist_ok=True)
    with open(mock_mutations, "w", encoding="utf-8") as mf:
        json.dump([{"test": "t1", "platforms": ["linux"]}], mf)
        
    try:
        with pytest.raises(ValueError) as excinfo:
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
        assert "No platform intersection" in str(excinfo.value)
    finally:
        os.remove(p)
        os.remove(mock_mutations)
