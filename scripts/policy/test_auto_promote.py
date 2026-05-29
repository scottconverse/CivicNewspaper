import json
import hashlib
import tempfile
import os
import shutil
import pytest
from unittest.mock import patch, MagicMock
from auto_promote import audit_team_zero_blockers

def mock_subprocess_run(cmd, *args, **kwargs):
    mock_res = MagicMock()
    sha = cmd[-1]
    if sha == "c0ffee12345":
        mock_res.returncode = 0
    else:
        mock_res.returncode = 1
    return mock_res

def _compute_sha256(filepath):
    sha256_hash = hashlib.sha256()
    with open(filepath, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

@pytest.fixture
def temp_run_env():
    temp_dir = tempfile.mkdtemp()
    audits_dir = os.path.join(temp_dir, "audits")
    os.makedirs(audits_dir)
    yield temp_dir, audits_dir
    shutil.rmtree(temp_dir)

def write_json(path, data):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f)

def write_text(path, text):
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)

def test_empty_file(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {})
    sha = _compute_sha256(p)
    with pytest.raises(ValueError):
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)

def test_missing_blockers(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    sha = _compute_sha256(p)
    with pytest.raises(ValueError):
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)

def test_blockers_one(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 1, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), "")
    sha = _compute_sha256(p)
    with pytest.raises(ValueError):
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)

def test_criticals_one(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 1, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), "")
    sha = _compute_sha256(p)
    with pytest.raises(ValueError):
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)

def test_verdict_block(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'BLOCK', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), "")
    sha = _compute_sha256(p)
    with pytest.raises(ValueError):
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)

def test_wrong_sha(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    with pytest.raises(Exception) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha="wrong-sha")
    assert "hash mismatch" in str(excinfo.value)

def test_blockers_string(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': "0", 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "blockers must be an integer" in str(excinfo.value)

def test_blockers_null(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': None, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "blockers must be an integer" in str(excinfo.value)

def test_passes_correct(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    
    sha = _compute_sha256(p)
    res = audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert res == "PASS"

def test_mutation_platforms_intersection_failure(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['windows'],
        'checkpoint_audits_clean': True
    })
    sha = _compute_sha256(p)
    
    mock_mutations = "scripts/audit/mutations.json"
    os.makedirs(os.path.dirname(mock_mutations), exist_ok=True)
    with open(mock_mutations, "w", encoding="utf-8") as mf:
        json.dump([{"test": "t1", "platforms": ["linux"]}], mf)
        
    try:
        with pytest.raises(ValueError) as excinfo:
            audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
        assert "No platform intersection" in str(excinfo.value)
    finally:
        if os.path.exists(mock_mutations):
            os.remove(mock_mutations)

def test_checkpoint_audits_clean_missing(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux']
    })
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "checkpoint_audits_clean" in str(excinfo.value)

def test_checkpoint_audits_clean_false(temp_run_env):
    temp_dir, _ = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': False
    })
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "checkpoint_audits_clean must be True" in str(excinfo.value)

def test_checkpoint_shas_txt_missing(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    shutil.rmtree(audits_dir)
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "checkpoint-shas.txt missing" in str(excinfo.value)

def test_checkpoint_file_missing(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), "some-sha  missing-checkpoint.md\n")
    sha = _compute_sha256(p)
    with pytest.raises(Exception) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "Checkpoint file missing" in str(excinfo.value)

def test_checkpoint_file_sha_mismatch(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"wrong-sha  {chk_file}\n")
    sha = _compute_sha256(p)
    with pytest.raises(Exception) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "Checkpoint file hash mismatch" in str(excinfo.value)

def test_checkpoint_file_has_blocker(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 1\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "Non-zero blockers or criticals count in" in str(excinfo.value)

def test_checkpoint_file_has_critical(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 1\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "Non-zero blockers or criticals count in" in str(excinfo.value)

def test_severity_rollup_regex_anchored(temp_run_env):
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    
    # Valid format: must PASS
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n- Major: 0\n- Minor: 0\n- Nit: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    sha = _compute_sha256(p)
    assert audit_team_zero_blockers(verdict_path=p, expected_sha=sha) == "PASS"

    # Non-anchored or malformed formats: must FAIL
    # Case 1: missing starting dash
    write_text(chk_file, "## Severity rollup\nBlocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "No blocker count found" in str(excinfo.value)

    # Case 2: extra text before the dash
    write_text(chk_file, "## Severity rollup\nExtra - Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "No blocker count found" in str(excinfo.value)

    # Case 3: extra text after the count
    write_text(chk_file, "## Severity rollup\n- Blocker: 0 extra\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "No blocker count found" in str(excinfo.value)

@patch("subprocess.run")
def test_citation_valid_url(mock_run, temp_run_env):
    mock_run.side_effect = mock_subprocess_run
    
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    
    # Create stage report
    report_file = os.path.join(temp_dir, "stage-1-report.md")
    write_text(report_file, "This is by design.\nSee details: https://example.com/valid-url\n")
    
    sha = _compute_sha256(p)
    assert audit_team_zero_blockers(verdict_path=p, expected_sha=sha) == "PASS"

@patch("subprocess.run")
def test_citation_url_accepted_without_network_check(mock_run, temp_run_env):
    # ENG-012: _is_valid_url is structural-only — it no longer issues a live
    # HEAD/GET, so a well-formed http(s) URL with a real path is accepted as a
    # citation even if the endpoint would 404. Network liveness is deliberately
    # NOT verified (non-deterministic in CI, slow, SSRF surface). This pins that
    # contract so a future change can't quietly reintroduce a network probe.
    mock_run.side_effect = mock_subprocess_run

    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })

    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")

    # A structurally valid URL with a path — would 404 over the network, but the
    # gate only checks structure now, so it counts as a valid citation.
    report_file = os.path.join(temp_dir, "stage-1-report.md")
    write_text(report_file, "This is by design.\nSee details: https://example.com/404-url\n")

    sha = _compute_sha256(p)
    assert audit_team_zero_blockers(verdict_path=p, expected_sha=sha) == "PASS"

@patch("subprocess.run")
def test_citation_generic_url(mock_run, temp_run_env):
    mock_run.side_effect = mock_subprocess_run
    
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    
    # Create stage report with generic URL (root path)
    report_file = os.path.join(temp_dir, "stage-1-report.md")
    write_text(report_file, "This is by design.\nSee details: https://github.com/\n")
    
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "lacks valid cited evidence" in str(excinfo.value)

@patch("subprocess.run")
def test_citation_fabricated_sha(mock_run, temp_run_env):
    mock_run.side_effect = mock_subprocess_run
    
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    
    # Create stage report with fabricated SHA
    report_file = os.path.join(temp_dir, "stage-1-report.md")
    write_text(report_file, "This is by design.\nSHA: fabricated12345\n")
    
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "lacks valid cited evidence" in str(excinfo.value)

@patch("subprocess.run")
def test_citation_valid_sha(mock_run, temp_run_env):
    mock_run.side_effect = mock_subprocess_run
    
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    
    # Create stage report with valid SHA (c0ffee12345 matches mock)
    report_file = os.path.join(temp_dir, "stage-1-report.md")
    write_text(report_file, "This is by design.\nSHA: c0ffee12345\n")
    
    sha = _compute_sha256(p)
    assert audit_team_zero_blockers(verdict_path=p, expected_sha=sha) == "PASS"

@patch("subprocess.run")
def test_citation_no_citation(mock_run, temp_run_env):
    mock_run.side_effect = mock_subprocess_run
    
    temp_dir, audits_dir = temp_run_env
    p = os.path.join(temp_dir, "verdict.json")
    write_json(p, {
        'blockers': 0, 'criticals': 0, 'verdict': 'PROMOTE', 'audit_artifact': 'x', 'auditor': 'x',
        'mutation_checks_results_sha256': 'some-sha',
        'clippy_platforms_run': ['linux'], 'mutation_checks_platforms': ['linux'],
        'checkpoint_audits_clean': True
    })
    
    chk_file = os.path.join(temp_dir, "checkpoint-1.md")
    write_text(chk_file, "## Severity rollup\n- Blocker: 0\n- Critical: 0\n")
    chk_sha = _compute_sha256(chk_file)
    write_text(os.path.join(audits_dir, "checkpoint-shas.txt"), f"{chk_sha}  {chk_file}\n")
    
    # Create stage report without citation
    report_file = os.path.join(temp_dir, "stage-1-report.md")
    write_text(report_file, "This is by design.\nNo citation here.\nAnd none here either.\n")
    
    sha = _compute_sha256(p)
    with pytest.raises(ValueError) as excinfo:
        audit_team_zero_blockers(verdict_path=p, expected_sha=sha)
    assert "lacks valid cited evidence" in str(excinfo.value)

