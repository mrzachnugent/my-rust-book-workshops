use adder;

mod common;
// To test this file:
// `cargo test --test integration_test`

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}