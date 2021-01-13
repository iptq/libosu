libosu
======

[![CI](https://travis-ci.org/iptq/libosu.svg?branch=master)](https://travis-ci.org/iptq/libosu)
[![Crates.io](https://img.shields.io/crates/d/libosu.svg)][1]
[![Documentation](https://docs.rs/libosu/badge.svg)][2]
[![dependency status](https://deps.rs/repo/github/iptq/libosu/status.svg)][3]

General-purpose osu! library

Installation
------------

This package is hosted on [crates.io][1]. In order to include this library into
your project, simply add this line into your `Cargo.toml`:

```rust
libosu = "*"
```

The following features are available through adding features in `Cargo.toml`,
and are not included by default since they may bring in extra dependencies:

- `apiv1`: Bindings for the osu! API v1.
- `replay`: osu! Replay parser

Getting Started
---------------

Check out the [API Documentation][2] for details on how to use the various
functions, or check out some of the examples (pending).

Projects using libosu
---------------------

- [editor](https://github.com/iptq/editor): wip osu editor

If you have a project using libosu, open an issue with a brief description and
I'll add it to the list!

Why Rust?
---------

The real question is, why _not_ rust? For a low level language, Rust has many
language features such as generics and a rich type system that greatly enhances
development. Additionally, its strong emphasis on memory safety means that it
can perform at native speeds. Most languages support native library extensions
already, so integration into other languages is also possible. One other
interesting feature is that Rust already has relatively good support for
WebAssembly, which means it could be possible to integrate this library into
web applications as well.

Contact
-------

Authors: Michael Zhang

License: MIT

[1]: https://crates.io/crates/libosu
[2]: https://docs.rs/libosu
[3]: https://deps.rs/repo/github/iptq/libosu
