# Contributing to rdocx

rdocx is a complete Word document workflow library for Rust: create, open, edit, validate, save DOCX packages and render to PDF, images, HTML, Markdown and more. It integrates parsing, layout, rendering and export in one stack without external Office dependencies.

See [tuna-os/.github/CODE_OF_CONDUCT.md](https://github.com/tuna-os/.github/blob/main/CODE_OF_CONDUCT.md) for community guidelines.

## Set Up

You need Rust 1.93 or later (see MSRV badge in README):

```bash
rustup update
git clone https://github.com/tuna-os/rdocx
cd rdocx
```

Optional system dependencies for additional formats:
- **PDF/image output**: See feature documentation for platform-specific font/layout libraries
- **Encryption/signing**: OpenSSL headers (feature-gated)

## Build and Test

```bash
# Build main library and all crates
cargo build --workspace

# Run test suite
cargo test --workspace

# Test specific feature combinations
cargo test --features encryption,signing
cargo test -p rdocx-layout   # layout engine tests
cargo test -p rdocx-render   # rendering tests

# Check code with clippy
cargo clippy --workspace --all-targets

# Format code
cargo fmt
```

Crates are organized by layer:
- `rdocx` — main facade for applications
- `rdocx-opc` — OPC package handling
- `rdocx-wml` — WordprocessingML parsing and writing
- `rdocx-layout` — flow-to-positioned-pages engine
- `rdocx-render` — render positioned pages to fixed formats (PDF, PNG, etc.)
- `rdocx-html` — flow-to-HTML conversion

Tests include unit tests for each layer, round-trip tests (read/write/read), and comparison tests against reference documents.

## Architecture Overview

rdocx follows a layered architecture:

1. **OPC** — reads/writes DOCX as zip archive, validates package structure
2. **WordprocessingML** — parses XML into document model, preserves unknown safe content
3. **Layout** — resolves flow (paragraphs, tables, runs) into positioned pages
4. **Rendering** — renders positioned pages to PDF, images (PNG/TIFF/JPEG/SVG)
5. **Export** — converts to HTML, Markdown, RTF, ODT, EPUB

Preservation is key: unknown XML elements are stored and retained on save, so editing a document with new content doesn't lose old features.

## Code Conventions

- Follow Rust style via `cargo fmt`
- Run `cargo clippy` to catch common mistakes
- Document public types and methods with `///` comments
- Preservation logic must be robust: unknown elements must survive edit/save cycles
- Font handling is deterministic: bundled fonts are used by default, system/embedded fonts are fallbacks
- All layout output must be reproducible (same input → same output regardless of environment)

## Pull Requests

1. Branch from `main`: `git checkout -b feature/your-feature`
2. Write commit messages: `feat: …`, `fix: …`, `refactor: …`, `docs: …`
3. Build and test: `cargo test --workspace && cargo clippy --workspace --all-targets && cargo fmt`
4. For layout or rendering changes, include before/after PDF or image samples
5. Push and open a PR against `main`

For significant architectural changes or new export formats, file an issue first to discuss.

## Publishing

rdocx is published to [crates.io](https://crates.io/crates/rdocx). Releases are tagged as `v<semver>` and follow semantic versioning.

---

By contributing, you agree your contributions are licensed under MIT/Apache-2.0.
