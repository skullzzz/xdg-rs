**This library has been moved to [xdg-basedir](https://github.com/kiran-kp/xdg-basedir).**

xdg-rs
====

[![Build Status](https://travis-ci.org/skullzzz/xdg-rs.svg)](https://travis-ci.org/skullzzz/xdg-rs) [![](http://meritbadge.herokuapp.com/xdg-rs)](https://crates.io/crates/xdg-rs)

[Documentation](http://skullzzz.github.io/xdg-rs/xdg/index.html)

xdg-rs is a utility library to make conforming to the
[XDG basedir specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier.

#Example
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

*Unstable features:*
- Test runtime directory: A function to check if a directory satisfies the XDG spec's requirements of a runtime directory.

The default build of xdg-rs does not use any unstable libstd features. To enable them, you'll need to use the nightly build of rustc and build xdg-rs with the 'unstable' feature toggle.

```toml
[dependencies.xdg-rs]
version = "0.1.3"
features = ["unstable"]
```

Alternate implementation and some initial source borrowed from [rust-xdg](https://github.com/o11c/rust-xdg).
The APIs provided by ```rust-xdg``` and ```xdg-rs``` are different.
