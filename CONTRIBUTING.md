# Contributing to Ketzal

> Fast. Elegant. Rust. — We're glad you're here.

Contributions of all kinds are welcome: features, bug fixes, documentation, performance work, and tests. This guide covers everything you need to get started the right way.

---

## Getting Started

**1. Fork & clone**

```bash
git clone https://github.com/YOUR_USERNAME/ketzal.git
cd ketzal
```

**2. Create a branch** using one of the standard prefixes:

```bash
git checkout -b feat/your-feature-name
```

| Prefix | Use for |
|---|---|
| `feat/` | New feature |
| `fix/` | Bug fix |
| `docs/` | Documentation |
| `refactor/` | Internal restructuring |
| `perf/` | Performance improvements |

---

## Making Changes

Build, test, and lint before submitting anything:

```bash
cargo build
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

Your PR is ready when:

- Code compiles without errors
- All tests pass
- No clippy warnings
- Properly formatted (`cargo fmt`)

---

## Commit Format

Ketzal follows [Conventional Commits](https://www.conventionalcommits.org/).

```
<type>(<scope>): <short message>
```

**Types**

| Type | Use for |
|---|---|
| `feat` | New feature or API |
| `fix` | Bug fix |
| `refactor` | Code restructuring (no behavior change) |
| `perf` | Performance improvement |
| `docs` | Documentation only |
| `test` | Add or modify tests |
| `style` | Formatting only |
| `chore` | Dependencies or tooling |
| `ci` | CI/CD configuration |

**Scopes:** `core` · `http` · `router` · `validation` · `macros` · `docs` · `ci` · `examples`

**Good examples**

```text
feat(router): add named route support
fix(validation): prevent duplicate rule execution
perf(http): reduce header allocation
docs(readme): improve installation section
```

**Avoid**

```text
fix bug.        ← no scope, ends with period
Added stuff     ← past tense, vague
misc changes    ← not conventional
```

---

## Submitting a Pull Request

```bash
git add .
git commit -m "feat(router): add middleware support"
git push origin feat/your-feature-name
```

Then open a PR on GitHub. A good PR:

- Explains **what** changed and **why**
- Includes tests when applicable
- Stays focused — no unrelated changes

---

## Code Style

- Follow standard Rust conventions
- Keep public APIs minimal, explicit, and stable
- Avoid unnecessary allocations
- Prefer zero-cost abstractions

---

## Reporting Bugs

Include in your issue:

- Steps to reproduce
- Expected vs actual behavior
- Rust version, OS, and Ketzal version

---

## Feature Requests

Before proposing, check: does it fit Ketzal's philosophy? Is it zero-cost when unused?

Include a problem description, proposed solution, and a usage example.

---

## Recognition

All contributors are listed in [CONTRIBUTORS.md](CONTRIBUTORS.md). Significant contributions may be highlighted in release notes.

---

<div align="center">

BSD-3-Clause · Thank you for contributing to Ketzal

</div>
