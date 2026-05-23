# Contributing to CivicNewspaper

Thanks for considering a contribution. CivicNewspaper is pre-alpha; the most valuable contributions right now are the small, concrete ones: a regex that catches your city's meeting-notice phrasing, a bug report with a reproducer, a doc fix.

## Ground rules

- The project is single-editor, local-first, no-cloud. Contributions that add cloud dependencies, telemetry, or analytics will be rejected.
- Every fact in published output must be linkable to a primary record. Contributions that weaken the guardrails (citation, presumption-of-innocence) need a strong justification.
- No vendored binaries. No bundled model weights. Ollama is a runtime dependency the user installs themselves.

## Development setup

See the "Building from source" section of [README.md](README.md) for prerequisites. Quick path once those are installed:

```bash
git clone https://github.com/scottconverse/CivicNewspaper.git
cd CivicNewspaper
npm install
npm run tauri dev
```

For backend-only work you can run the Rust tests without launching Tauri:

```bash
cd src-tauri
cargo test
```

## Where to start

### Easy: improve a detector regex

`src-tauri/src/core/detectors.rs` defines eight regex-based detectors. Each one is a single `Regex::new(...)` line. If your local government uses phrasing the current regexes miss, that's a real bug — open a PR that:
1. Adds your phrase to the regex.
2. Adds a unit test in `src-tauri/src/core/tests.rs` exercising your phrase.
3. Includes a one-line note in the PR description naming the municipality (so reviewers can sanity-check that the term is genuinely civic boilerplate, not editorial language).

### Easy: improve a guardrail keyword

`src-tauri/src/core/guardrails.rs` has a hard-coded `accusatory_words` list. If your editorial judgment says a word belongs in or out, open an issue first to discuss — this list is editorial policy, not just code.

### Medium: frontend componentization

`src/App.tsx` is a 1,918-line single-page React component. Pulling out reusable pieces (the Queue view, the Workbench, the Onboarding wizard, the Settings tab) into `src/components/` would dramatically improve maintainability. The first PR on this path should pick one tab only and ship a working extraction with no behavior change.

### Harder: real NLP for detectors

The current detector layer is regex. There is a credible case for layering a small local NER model on top — but the bar is high. A proposal-stage issue should answer:
- What model, what footprint, what latency?
- How does it degrade on machines that can't run it?
- How is it kept honest (regression tests against the existing regex outputs)?

## Pull request style

- One logical change per PR.
- Include tests for backend logic changes.
- Cosmetic-only changes are exempt from the test requirement but should be in their own PR.
- Run `cargo fmt` for Rust and `npm run lint` (if/once configured) for TS before pushing.
- PR description: what changed, why, and how you verified it.

## Commit messages

Plain, imperative-mood, descriptive. No conventional-commits prefix is required; do not gate PRs on commit-message style.

Examples that are fine:
- `detectors: add 'unanimous consent' to vote regex`
- `guardrails: skip code fences when checking for evidence: links`
- `docs: fix broken file:// links in README`

## Filing issues

For bugs: a minimal reproducer is worth ten paragraphs of description. If you can paste the offending source text and the wrong-output (or the missing lead), do that.

For feature requests: state the user need before the feature. "My city's water board posts agendas as PDFs and we miss them" is more useful than "add PDF support."

## Code of conduct

By participating you agree to follow the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). Be kind. Assume good faith. Reviewers can be slow; the project has no full-time maintainer.

## Licensing

By contributing you agree your contribution is licensed under the MIT License (see [LICENSE](LICENSE)).
