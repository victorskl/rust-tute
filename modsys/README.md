# modsys

Rust [Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

```
mkdir -p modsys
cd modsys

cargo new --lib restaurant
cd restaurant
cargo build
./target/debug/restaurant
./target/debug/my_cmd
```

**Recap**:

> Code Organisation: `code < module < crate < package`

- A crate is a collection of Rust source code files.
- A crate can be a binary (executable) or library (reusable).
- A package has one or more crates.
- A package contains a `Cargo.toml` file that describes how to build those crates. 🙋‍♂️ _Hence, [restaurant](restaurant) is a package._
- A package must contain **zero or one library crate**, and no more.
    > Convention: `src/lib.rs`
- A package can contain **as many binary crates as** you'd like, but it **must contain at least one crate** (either library or binary).
    > Convention: `src/main.rs` and more under `src/bin/`
- If you want to make an item like a function or struct private, you put it in a module.
- A module start with `mod` keyword follow by curly bracket code block i.e. 
```rust
  mod my_mod_name {  
      // fn, struct, impl, enum
  }
```
- Modules can be nested and multiple modules form **_a module tree_** and, `crate` module is the implicit module root -- (_think of [Java Object class](https://docs.oracle.com/en/java/javase/15/docs/api/java.base/java/lang/Object.html)_). i.e.
```shell script
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
- Nested modules form _parent and child module hierarchy_ and, use `super` keyword to call parent module function. i.e.
```rust
fn some_common_fn_at_my_module_aggregate_root() {}
mod some_mod {
    fn some_foo() {
        super::some_common_fn_at_my_module_aggregate_root();
    }
}
```
- Function in parent module can not call child (nested) module's some function without being scope it to public.
- Module scope is **Private by default**. Use `pub` keyword to expose them.
- Using keyword `use` and `as` to shorten module import and, `pub use` to re-export module. Some examples:
```rust
use std::io;
use std::io::Write;
// can be shorten to
use std::io::{self, Write};

// ---

use std::cmp::Ordering;
use std::io;
// can be shorten to
use std::{cmp::Ordering, io};

// ---

// Use `as` to alias new name for importing item
use std::fmt::Result;
use std::io::Result as IoResult;

// ---

// Use glob operator `*` to bring in all!
use std::collections::*;
```
- Using a semicolon after `mod some_module` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module.
