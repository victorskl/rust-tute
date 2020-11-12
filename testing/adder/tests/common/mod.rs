// tests/common/mod.rs  <-- This is an alternate naming convention that Rust also understands.
// Naming the file this way tells Rust not to treat the common module as an integration test file.
// https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#submodules-in-integration-tests

pub fn setup() {
    // setup code specific to your library's tests would go here
    println!("Common integration tests setup is called!")
}

// cargo test -- --show-output
