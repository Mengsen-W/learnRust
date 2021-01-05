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

#![feature(proc_macro_non_items)]
use simple_proc_macro::hashmap;
#[test]
fn test_hashmap () {
    let hm = hashmap!{"a": 1, "b": 2};
    assert_eq!(hm["a"], 1);
    let hm = hashmap!{"a" => 1, "b" => 2, "c" => 3};
    assert_eq!(hm["d"], 4);
}
