use adder;

mod common;

#[test]
fn it_it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// cargo test --test integration_test
