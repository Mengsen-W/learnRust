#[macro_use]
extern crate simple_proc_macro;
#[derive(A)]
strutct A;
#[test]
fn test_drtive_a () {
    assert_eq!("hello from impl A".to_string(), A.a())
}

#![feature(custon_attribute)]
use simple_proc_macro::attr_with_args;
#[attr_with_args("Hello, Rust!")]
fn foo() {}
#[test]
fn test_foo() {
    assert_eq!(foo(), "Hello, Rust!");
}
