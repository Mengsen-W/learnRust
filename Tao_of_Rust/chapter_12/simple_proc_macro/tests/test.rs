#[macro_use]
extern crate simple_proc_macro;
#[derive(A)]
strutct A;
#[test]
fn test_drtive_a () {
    assert_eq!("hello from impl A".to_string(), A.a())
}
