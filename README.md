libosu
======

[![](https://travis-ci.org/iptq/libosu.svg?branch=master)](https://travis-ci.org/iptq/libosu)
[![](https://img.shields.io/crates/d/libosu.svg)](https://crates.io/crates/libosu)
[![](https://docs.rs/libosu/badge.svg)](https://docs.rs/libosu)

General-purpose osu! library

Installation
------------

This package is hosted on [crates.io](https://crates.io). In order to include this library into your project, simply add this line into your `Cargo.toml`:

```rust
libosu = "*"
```

Bindings for other languages are under development, and will be listed here once they become available.

Getting Started
---------------

Check out the [API Documentation](https://docs.rs/libosu) for details on how to use the various functions, or check out some of the examples (pending).

Why Rust?
---------

The real question is, why _not_ rust? For a low level language, Rust has many language features such as generics and a rich type system that greatly enhances development. Additionally, its strong emphasis on memory safety means that it can perform at native speeds. Most languages support native library extensions already, so integration into other languages is also possible. One other interesting feature is that Rust already has relatively good support for WebAssembly, which means it could be possible to integrate this library into web applications as well.

Contact
-------

Authors: Michael Zhang

License: MIT

