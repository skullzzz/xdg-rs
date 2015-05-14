xdg-rs
====

[![Build Status](https://travis-ci.org/skullzzz/xdg-rs.svg)](https://travis-ci.org/skullzzz/xdg-rs) [![](http://meritbadge.herokuapp.com/xdg-rs)](https://crates.io/crates/xdg-rs)

[Documentation](http://skullzzz.github.io/xdg-rs/xdg/index.html)

xdg-rs is a utility library to make conforming to the
[XDG basedir specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier.

##Example
```rust
#![cfg(unix)]
extern crate xdg;

#![cfg(unix)]
use xdg;
use std::path::PathBuf;
...
let data_home: PathBuf = try!(xdg::get_data_home());
...
```

Some functions require the crate to be compiled with nightly rustc build and unstable libstd features. Build with the 'nightly' feature toggle to enable these functions.

```toml
[dependencies.xdg-rs]
version = "0.1.2"
features = ["nightly"]
```

Alternate implementation and some initial source borrowed from [rust-xdg](https://github.com/o11c/rust-xdg).
The APIs provided by ```rust-xdg``` and ```xdg-rs``` are different.
