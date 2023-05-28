# Cross Build

https://github.com/rust-embedded/cross

> **TL;DR** Rust is like C/C++ which need to be compiled for the target platform. The `cross` approach is just building Rust binary inside Docker container for the target cross platform/s. 

```
cargo search cross
cargo install cross
which cross
cross --help
```

Let's try some cross build:
```shell script
cargo new xcli
cd xcli

# It just works like `cargo build`
cross build
tree target/
./target/debug/xcli
```

This `./target/debug/xcli` build is only work for the platform where you are developing on, e.g. macOS. We can use a couple of tools to inspect the compiled binary object for which target platform they are built against.
```shell script
file ./target/debug/xcli
objdump -p ./target/debug/xcli
otool -l ./target/debug/xcli
gobjdump -p ./target/debug/xcli
```

Let change it to different target platform, i.e. `cross` will pull docker image and build for that platform. After build has done inside container, `cross` spit out the build artefacts into the `./target/<platform-target-name>/` directory.
```shell script
cross build --target aarch64-unknown-linux-gnu

file ./target/aarch64-unknown-linux-gnu/debug/xcli
gobjdump -p ./target/aarch64-unknown-linux-gnu/debug/xcli
greadelf -a ./target/aarch64-unknown-linux-gnu/debug/xcli | less
```

Observe that `cross` has pull docker image for the target platform.
```
docker images | grep cross
```

From ☝️, the _aarch64_ is [ARM64 architecture](https://en.wikipedia.org/wiki/AArch64). For developing server side software, we will be building for [x86_64](https://en.wikipedia.org/wiki/X86-64) arch platforms; either in [GNU glibc](https://en.wikipedia.org/wiki/GNU_C_Library) on typical OS like CentOS, Debian; or [musl](https://en.wikipedia.org/wiki/Musl) for Alpine.
```shell script
cross build --target x86_64-unknown-linux-gnu
file ./target/x86_64-unknown-linux-gnu/debug/xcli

cross build --target x86_64-unknown-linux-musl
file ./target/x86_64-unknown-linux-musl/debug/xcli
```

The `Cross.toml` file contains the [configuration](https://github.com/rust-embedded/cross#configuration) for `cross`.

Cross ship pre-built [docker images](https://github.com/rust-embedded/cross/tree/master/docker). However, you can use [custom image](https://github.com/rust-embedded/cross#custom-docker-images) for a particular target platform, if any.

### Testing

```shell script
# Normal Cargo test
cargo test

# This is identical to cargo test
cross test

# Test against target platforms
cross test --target x86_64-unknown-linux-gnu
cross test --target x86_64-unknown-linux-musl
```
