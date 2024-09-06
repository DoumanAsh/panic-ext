# panic-ext

[![Rust](https://github.com/DoumanAsh/panic-ext/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/panic-ext/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/panic-ext.svg)](https://crates.io/crates/panic-ext)
[![Documentation](https://docs.rs/panic-ext/badge.svg)](https://docs.rs/crate/panic-ext/)

Extension library to panic facilities to make it more usable

Requires Rust 1.81

## Features

- `alloc` - Enables usage of `alloc` types
- `std` - Enables `std::error::Error` impl on panic details. Implies `alloc`
