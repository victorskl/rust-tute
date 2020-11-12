# Testing

- https://doc.rust-lang.org/cargo/guide/tests.html
- https://doc.rust-lang.org/stable/book/ch11-00-testing.html

```
cargo new foo
cd foo
cargo test
cargo test foo
```

```
cargo new adder --lib
cd adder
cargo test
```

```
cargo test --help
cargo test -- --help
cargo test -- --show-output
cargo test this_test_will_pass -- --show-output
cargo test one_hundred
cargo test add
cargo test -- --ignored
cargo test --test integration_test
cargo test --test integration_test -- --show-output
```

**Recap:**

- A test in Rust is a function.
- To change a function into a test function, annotate with  `#[test]` on the line before `fn` e.g.
```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```
- **Unit Tests**: in each of your `src/` files
    - The convention is to create a module named `mod tests` in each file to contain the test functions and to annotate the module with `#[cfg(test)]`.
    - Unit tests go in the same files as the code, you'll use `#[cfg(test)]` to specify that they shouldn't be included in the compiled result.
- **Integration Tests**: any tests in `tests/` directory
    - Need to _import crates into the files in `tests/`_
    - They don't need the `#[cfg(test)]` annotation because integration tests go in a different directory.
    - We can make as many test files as we want to in this directory, and Cargo will compile each of the files as an individual crate.
    - To run all the tests in a particular integration test file, use the `--test` argument of `cargo test` followed by the name of the file.
    - [No integration tests for Binary Crate](https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#integration-tests-for-binary-crates)!! If need one, refactor those functions into `lib.rs` and preform integration tests from this library crate.
- Need to derive test traits if struct or enum
> - When the assertions fail, assertion macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. 
> - All the primitive types and most of the standard library types implement these traits. 
> - For **structs** and **enums** that you define, you'll need to implement `PartialEq` to assert that values of those types are equal or not equal. You'll need to implement `Debug` to print the values when the assertion fails. 
> - Because both traits are derivable traits, this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition.
- Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
- Because the tests are running at the same time, make sure your tests don't depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

**Assertion Marcos:**

```
panic!(MESSAGE_STR)
format!(MESSAGE_STR_WITH_PLACEHOLDER, SUBSTITUTE)
assert!(BOOL_EXP)
assert_eq!(LEFT, RIGHT)
assert_ne!(LEFT, RIGHT)
```
