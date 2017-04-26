# rdoc

A simple rustc plugin to host documentation for your items in a separate file.

[![Build Status](https://travis-ci.org/mgattozzi/rdoc.svg?branch=master)](https://travis-ci.org/mgattozzi/rdoc)
[![Build
Status](https://ci.appveyor.com/api/projects/status/06bp8wk087geswk/branch/master?svg=true)](https://ci.appveyor.com/project/mgattozzi/rdoc/branch/master)
[![crates.io](https://img.shields.io/crates/v/rdoc.svg)](https://crates.io/crates/rdoc)

## Nightly
As this is a compiler plugin Rust Nightly is required

## How to use it

In your `Cargo.toml` put:

```toml
[dependencies]
rdoc = "*"
```

In your `lib.rs` or `main.rs` file place the following at the top:

```rust
#![feature(plugin)]
#![plugin(rdoc)]
```

Given a file `doc.md` living at the top of your directory, in your code place
this over the item you want to be documented:

```rust
#[rdoc = "doc.md"]
pub fn print_hello() {
    println!("Hello!");
}
```

At compile time this annotation will turn into a doc comment with the contents
of `doc.md` for whatever item is below it. No need for specific file types or
putting the `///` annotations in the file. Just write your documentation text
and `rdoc` takes care of the rest for you! All you need to do is provide a valid
path in the annotation.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE)
   or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
