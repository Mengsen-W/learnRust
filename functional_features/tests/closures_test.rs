use functional_features;

#[test]
fn call_with_different_values() {
    let mut c = functional_features::closures::Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);

}
