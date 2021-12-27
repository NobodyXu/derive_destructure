# derive_destructure2 **(no-std)**

[![Rust](https://github.com/NobodyXu/derive_destructure2/actions/workflows/rust.yml/badge.svg)](https://github.com/NobodyXu/derive_destructure2/actions/workflows/rust.yml)

[![crate.io downloads](https://img.shields.io/crates/d/derive_destructure2)](https://crates.io/crates/derive_destructure2)

[![crate.io version](https://img.shields.io/crates/v/derive_destructure2)](https://crates.io/crates/derive_destructure2)

[![docs](https://docs.rs/derive_destructure2/badge.svg)](https://docs.rs/derive_destructure2)

**[examples](https://docs.rs/derive_destructure2_examples)**

This crate allows you to destructure structs that implement `Drop`.

If you've ever struggled with error E0509
"cannot move out of type `T`, which implements the `Drop` trait"
then this crate may be for you.

To use this crate, put this in your `lib.rs` or `main.rs` for rust < 1.30:

```ignore
#[macro_use]
extern crate derive_destructure2;
```

For rust >= 1.30, just import it as a regular item:

```ignore
use derive_destructure2::{derive_destructure, remove_trait_impls};
```

Then you have 2 ways to use this crate:

## Option 1: `#[derive(destructure)]`

If you mark a struct with `#[derive(destructure)]`, then you can destructure it using
from your crate

```rust
let (field_1, field_2, ...) = my_struct.destructure();
```

This turns the struct into a tuple of its fields **without running the struct's `drop()`
method**. You can then happily move elements out of this tuple.

Note: in Rust, a tuple of 1 element is denoted as `(x,)`, not `(x)`.

__**`destructure` is implemented as a `pub(crate)` associated function.**__

## Option 2: `#[derive(remove_trait_impls)]`

If you mark your struct with `#[derive(remove_trait_impls)]`, then you can do
from your crate

```rust
let my_struct = my_struct.remove_trait_impls();
```

The result is a struct with the same fields, but it implements no traits
(except automatically-implemented traits like `Sync` and `Send`).
In particular, it doesn't implement `Drop`, so you can move fields out of it.

The name of the resulting struct is the original name plus the suffix
`WithoutTraitImpls`.

For example, `Foo` becomes `FooWithoutTraitImpls`. But you usually don't need to write
out this name.

`#[derive(remove_trait_impls)]` works on enums too.

__**`remove_trait_impls` is a `pub(crate)` associated function.**__

## Example:

```rust
use derive_destructure2::*;

#[derive(destructure, remove_trait_impls)]
struct ImplementsDrop {
    some_str: String,
    some_int: i32
}

impl Drop for ImplementsDrop {
    fn drop(&mut self) {
        panic!("We don't want to drop this");
    }
}

fn main() {
    // Using destructure():
    let x = ImplementsDrop {
        some_str: "foo".to_owned(),
        some_int: 4
    };
    let (some_str, some_int) = x.destructure();
    // x's drop() method never gets called

    // Using remove_trait_impls():
    let x = ImplementsDrop {
        some_str: "foo".to_owned(),
        some_int: 4
    };
    let x = x.remove_trait_impls();
    // this x doesn't implement drop,
    // so we can move fields out of it
    drop(x.some_str);
    println!("{}", x.some_int);
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
