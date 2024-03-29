# rust-tute

Assorted Rust tutes

## Getting Started

- [RustRover](https://www.jetbrains.com/rust/) or others with [IntelliJ Rust plugin](https://github.com/intellij-rust/intellij-rust#compatible-ides)
- Use [`rustup`](https://rustup.rs/) to install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
which cargo
which rustc
which rustup
cargo --version
rustc --version
rustup update
```

- [Hello World](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)
```
mkdir hello_world
cd hello_world
vi main.rs
rustc main.rs
./main
rustc main.rs -o main.exe
./main.exe
```

- Cargo [get-started](https://www.rust-lang.org/learn/get-started)
```
cargo new --help
cargo new --vcs none hello-rust
cd hello-rust
cargo run
./target/debug/hello-rust
tree .

(add 'ferris-says = "0.2"' to dependencies section in Cargo.toml and correspondent code in main.rs)

cargo clean
cargo check
cargo build
cargo run
tree -L 2 target
cargo build --release
tree -L 2 target
./target/release/hello-rust
```

### Recap
- The [program entrypoint](https://en.wikipedia.org/wiki/Entry_point#Rust) is a function named `main()`, typically in `main.rs`
- `rustup` -- the Rust installer and version management tool
- `cargo` -- the Rust build tool and package manager
- `Cargo.toml` -- Cargo config that use [toml format](https://en.wikipedia.org/wiki/TOML)
- `Cargo.lock` -- [git ignored for libraries, git tracked for binaries](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- Cargo prescribe [a convention for project layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) -- _much resemble to Maven_
- https://crates.io -- the Rust package registry
- `cargo doc --open` -- generate documentation including dependencies
- Also note about [Rust underscore vs hyphen](https://www.google.com/search?q=rust+underscore+vs+hyphen) -- _similar to [Python import hyphen](https://www.google.com/search?q=python+import+hyphen)_ -- a Rust module import/include valid identifier is **underscore** i.e. `use foo_module::bar_func`

### Using tool written in Rust

Just like others, such as `yarn global add <foo>` or `npm -g install <bar>` for some tool written in NodeJS eco-system; or `pip install <foo>` or `pipx install <bar>` for Python; or `go get github.com/some/xtool` for Go; etc... we can use Cargo install for some tool that is written in Rust.

```shell script
cargo install --help
cargo help install

# e.g. WebSocket cli client
cargo search websocat
cargo install websocat
which websocat
websocat --help
```

### Next

- [modsys](modsys) -- code organization and module system
- [primer](primer) -- language basic
- [testing](testing) -- code testing
- [cross-build](cross-build) -- building for different platforms
- [guessing_game](guessing_game) -- example app

## REF

- [Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Rust Book](https://doc.rust-lang.org/stable/book/)
- [Rust Learn](https://www.rust-lang.org/learn)
