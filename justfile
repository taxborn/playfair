# Task runner for playfair. `just ci` runs exactly what CI runs, in the same
# order, so a green run here means a green run there.
#
# Keep this in sync with .forgejo/workflows/ci.yml by hand. CI deliberately does
# not call `just`: it isn't packaged for Debian, so getting it into the build
# container would cost more time than the checks themselves take to run.
#
# On NixOS nothing provides a linker by default, and rustfmt/clippy ship as
# separate packages, so run these inside the shell.nix dev shell: `nix-shell` in
# the repo root, then `just ci`.

# Show the available recipes.
default:
    @just --list

# Run every check CI enforces.
ci: fmt-check lint build test doc

# Check formatting without modifying files.
fmt-check:
    cargo fmt --check

# Apply formatting.
fmt:
    cargo fmt

# Lint all targets, including benches, with the crate's strict-docs rule.
lint:
    cargo clippy --all-targets -- -F clippy::missing_docs_in_private_items -D warnings

# Compile everything, benches included.
build:
    cargo build --all-targets

# Run the tests; extra arguments pass through.
test *ARGS:
    cargo test {{ ARGS }}

# Build the docs, failing on broken intra-doc links.
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items

# Run the criterion benchmarks. Slow and noisy, so not part of `ci`.
bench:
    cargo bench
