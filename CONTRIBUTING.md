# Contributing to rdocx

Thank you for your interest in contributing to rdocx. This document explains how to set up your environment, run tests, and submit contributions.

## About rdocx

rdocx is a Rust library for the complete Word document workflow: create, open, edit, validate, and save DOCX files with deterministic layout and export to PDF, HTML, Markdown, and other formats. The project is built on a modular architecture with specialized crates for OPC, WordprocessingML, layout, and fixed output.

## Prerequisites

- **Rust 1.93 or newer** (check [MSRV badge](https://github.com/tuna-os/rdocx/blob/main/README.md))
- `cargo` and `rustup`
- For rendering tests: system fonts or deterministic font mode (see Testing below)

## Local Development Setup

### 1. Clone and prepare

```bash
git clone https://github.com/tuna-os/rdocx.git
cd rdocx
```

### 2. Build all crates

The workspace includes multiple crates (`rdocx`, `rpptx`, `oxml-*`, layout, HTML, etc.):

```bash
cargo build --workspace
```

### 3. Run tests

```bash
cargo test --workspace
```

### 4. Check code quality

```bash
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Workflow & Sprint Organization

rdocx uses sprint-driven development with tracked artifacts:

- `docs/sprints/CURRENT_SPRINT.md` - active sprint tracker
- `.claude/plans/` - design plans for features
- `.claude/reviews/` - design reviews
- `BACKLOG.md` - prioritized work queue
- `SPRINT_PLAN.md` - sprint scope
- `AS_BUILT.md` - delivered features record

If you're picking up a feature in progress, check `.claude/scratch/F-XXX-progress.md` for context.

## Core Rules

**These rules are non-negotiable and apply to all contributors:**

1. **Hash harness gates every PR.** Output deltas must be explained and reviewed. Behavioural changes must be in their own labelled commit stating the expected delta.

2. **Rendering baselines use deterministic font mode.** Never record a baseline against system fonts - results must be reproducible without local font installation.

3. **Crate dependency rules:**
   - `oxml-*` crates must NOT depend on `rdocx-*` or `rpptx-*` (except: `oxml-drawing → rdocx-oxml` for `Theme` adapter)
   - Preserve this separation - it enables reuse of XML layers

4. **Preserve unmodelled XML verbatim.** Parse only what you render. This ensures round-trip fidelity for producer content.

5. **Respect schema child order.** OOXML uses `xsd:sequence`. Violating order makes PowerPoint reject the file, not warn.

6. **Markdown style:**
   - Do not use em dashes (`-`) in tracked Markdown or commit messages
   - Do not use prose semicolons
   - Keep prose clear and concise

7. **Build hygiene:**
   - Do not run `cargo clean`
   - Iterate with scoped builds
   - Only `/release` may create `v*` tags or push to crates.io

## Commit Message Format

- Use conventional commits: `feat:`, `fix:`, `docs:`, `test:`, etc.
- State behavioural changes in the message - especially render output deltas
- Reference feature IDs (e.g., `F-123`) when applicable
- Sign commits with DCO: `git commit -s`

**Example:**
```
fix: preserve text direction in run properties

- Fixes F-042: RTL text now rendered correctly
- Updated rendering baseline (expected delta: +3 runs in RTL test)
- Crate: rdocx-layout
```

## Testing

### Unit tests

```bash
cargo test --workspace
```

### Rendering tests (deterministic fonts)

The project uses deterministic font mode for reproducible output. When adding rendering tests:

1. Generate baseline with deterministic fonts enabled
2. Update test expectations in the PR description
3. Include hash delta explanation in commit message

### Hash harness

The hash harness validates rendering output across crates:

```bash
python3 scripts/sync_agent_skills.py --check   # Validate agent skill adapters
```

## Documentation

- **README.md**: High-level overview and quick examples
- **src/lib.rs**: Crate-level documentation (examples, usage patterns)
- **AGENTS.md**: Agent and workflow guidance
- **CLAUDE.md**: Claude-specific instructions (for LLM agents)

When adding features, include doc comments on public APIs and examples in README or lib.rs.

## Pull Request Guidelines

1. **Target `main` branch** for most work. Check `.claude/WORKFLOW.md` for sprint-specific branches.

2. **PR title format:** `[feature-id] short description` or `[bug] fix description`
   - Example: `[F-123] add field hyperlink support`

3. **PR body should include:**
   - What changed and why
   - Any hash/rendering delta expected (with explanation)
   - Testing done
   - Related issues or feature IDs

4. **Keep PRs focused** - one feature or fix per PR

5. **All tests must pass** before merge:
   ```bash
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all -- --check
   ```

## Working with Sprint Features

If you're contributing to an in-progress feature:

1. **Check the feature state:** Read `.claude/scratch/F-XXX-progress.md` for context
2. **Understand design:** Review `.claude/plans/F-XXX-design.md` before implementation
3. **Update progress:** Keep `.claude/scratch/F-XXX-progress.md` current as you work
4. **Prepare handoff:** Use `/complete-feature --prepare` to generate `.claude/handoffs/F-XXX-ready.md` when done

## Questions?

- Check [AGENTS.md](AGENTS.md) for automation and workflow details
- Review `.claude/WORKFLOW.md` for process specifics
- Open a GitHub discussion for design questions
- File an issue for bugs or feature requests

## License

rdocx is dual-licensed under MIT and Apache-2.0. By contributing, you agree your code is licensed the same way.
