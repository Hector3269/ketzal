<div align="center">

```
██╗  ██╗███████╗████████╗███████╗ █████╗ ██╗
██║ ██╔╝██╔════╝╚══██╔══╝╚══███╔╝██╔══██╗██║
█████╔╝ █████╗     ██║     ███╔╝ ███████║██║
██╔═██╗ ██╔══╝     ██║    ███╔╝  ██╔══██║██║
     ██║  ██╗███████╗   ██║   ███████╗██║  ██║███████╗
     ╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝
```

**Laravel's elegance. Rust's performance.**

[![crates.io](https://img.shields.io/crates/v/ketzal.svg?style=flat-square&color=e07b39&label=crates.io)](https://crates.io/crates/ketzal)
[![docs.rs](https://img.shields.io/docsrs/ketzal?style=flat-square&color=4af0c8)](https://docs.rs/ketzal)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-c8f04a.svg?style=flat-square)](License)

</div>

---

## What is Ketzal?

Ketzal is a modern web framework for Rust designed for developers who want **expressive, readable code** without sacrificing the performance and safety Rust provides. If you know Laravel, you'll feel at home — routing, validation, and HTTP handling all follow familiar patterns, backed by Rust's type system and async runtime.

---
## Project Structure

A typical Ketzal application:

```
my-app/
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   └── web.rs                    ← route definitions
│   └── app/
│       └── http/
│           └── controllers/
│               └── user_controller.rs
└── Cargo.toml
```

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR.

See [CONTRIBUTORS.md](CONTRIBUTORS.md) for the full list of contributors.

---

<div align="center">

BSD-3-Clause · Built with 🦀 Rust

</div>
