# Development task runner
# Install: cargo install just
# Usage: just <recipe>

# Run all checks (mirrors CI)
check: fmt-check clippy test embedded doc
    @echo "All checks passed."

# Format code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all --check

# Run clippy lints
clippy:
    cargo clippy --all-features --all-targets -- -D warnings

# Run all tests
test:
    cargo test --all-features
    cargo test --no-default-features

# Verify no_std on a real embedded target
embedded:
    cargo check --target thumbv7em-none-eabihf --no-default-features

# Build documentation
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

# Quick dev check (fastest feedback loop)
dev:
    cargo check --all-features

# Run cargo deny checks
deny:
    cargo deny check

# MSRV check
msrv:
    cargo +1.75 check --all-features

# Prepare release (reads version from VERSION file, syncs to Cargo.toml)
release:
    #!/usr/bin/env bash
    set -euo pipefail
    VERSION=$(cat VERSION | tr -d '[:space:]')
    echo "Preparing release v${VERSION}..."
    sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml 2>/dev/null || \
    sed -i    "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
    cargo check --all-features
    echo "Ready. Review changes, commit, and tag with: git tag v${VERSION}"
