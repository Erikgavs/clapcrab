# Contributing to ClapCrab

Thanks for your interest in contributing! This project is open source under the MIT license.

## Getting Started

1. Fork the repository
2. Clone your fork
3. Make sure you have Rust installed (`rustup` recommended)
4. Run `cargo build` to verify everything compiles

## Branch Naming

Create a branch from `main` using these prefixes:

- `feature/` — new functionality (e.g. `feature/clap-detection`)
- `fix/` — bug fixes (e.g. `fix/threshold-sensitivity`)

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add double clap detection
fix: reduce false positives in noisy environments
docs: update README with usage instructions
refactor: extract audio processing into module
```

## Pull Requests

- One feature or fix per PR
- Write a clear description of what your PR does and why
- Make sure `cargo build` and `cargo clippy` pass before submitting

## Issues

Found a bug or have an idea? Open an issue first to discuss it before starting work.
