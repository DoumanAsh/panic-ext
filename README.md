# panic-ext

![Rust](https://github.com/DoumanAsh/panic-ext/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/panic-ext.svg)](https://crates.io/crates/panic-ext)
[![Documentation](https://docs.rs/panic-ext/badge.svg)](https://docs.rs/crate/panic-ext/)

Extension library to panic facilities to make it more usable

## Features

- `alloc` - Enables `String` usage via `alloc`. This is useful until [message](https://doc.rust-lang.org/std/panic/struct.PanicInfo.html#method.message) is stable
- `std` - Enables `std::error::Error` impl on panic details. Implies `alloc`
