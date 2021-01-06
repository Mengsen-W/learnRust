use derive_new::New;
// 无字段结构体
#[derive(New, PartialEq, Debug)]
pub struct Foo {}
// 包含字段的结构体
#[derive(New, PartialEq, Debug)]
pub struct Bar {
    pub x: i32,
    pub y: String,
}
// 单元结构体
#[derive(New, PartialEq, Debug)]
pub struct Baz;
// 元组结构体
#[derive(New, PartialEq, Debug)]
pub struct Tuple(pub i32, pub i32);

#[test]
fn test_empty_struct() {
    let x = Foo::new();
    assert_eq!(x, Foo {});
}
#[test]
fn test_simple_struct() {
    let x = Bar::new(42, "Hello".to_owned());
    assert_eq!(
        x,
        Bar {
            x: 42,
            y: "Hello".to_owned()
        }
    );
}
#[test]
fn test_empty_struct() {
    let x = Bar::new();
    assert_eq!(x, Bar);
}
#[test]
fn test_simple_struct() {
    let x = Tuple::new(5, 6);
    assert_eq!(x, Tuple(5, 6));
}
