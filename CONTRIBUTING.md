# Contributing

Thanks for your interest in contributing!

## Development

Install [just](https://github.com/casey/just) for the task runner, then:

```bash
# Run all checks (mirrors CI)
just check

# Quick feedback during development
just dev

# Format code
just fmt

# Run tests only
just test
```

**Note:** `just check` includes an embedded target check (`cargo check --target thumbv7em-none-eabihf`),
so you need to install the target first:

```bash
rustup target add thumbv7em-none-eabihf
```

## Supply-Chain Auditing

Run `just deny` to check dependencies against advisories and license policies.
This requires [cargo-deny](https://github.com/EmbarkStudios/cargo-deny):

```bash
cargo install cargo-deny
just deny
```

## Quality Gates

All of these must pass before merging:

- `cargo fmt --all --check` — consistent formatting
- `cargo clippy --all-features --all-targets -- -D warnings` — zero warnings
- `cargo test --all-features` — all tests pass
- `cargo test --no-default-features` — no_std tests pass
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` — docs build cleanly

## Versioning

The `VERSION` file is the single source of truth. It must match the version in `Cargo.toml`. CI enforces this on PRs.

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
