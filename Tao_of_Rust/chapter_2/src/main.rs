/// Chapter 2 learning
/// mengsen mengsen_wang@163.com
/// 2020-11-8 22:25

fn main() {
    println!("{:?}", no_return_value());
    let a = &10;
    print_address(a);
}

fn no_return_value() -> () {
    // return unit type
    // mean no return value in this function
    let a = 40;
    let b = 2;
    assert_eq!((a + b), 42);
    // statement return and expression return all mean return
    return ();
    // Is equivalent to
    // return ()
}

fn print_address(a: &u32) {
    println!("memory address = {:p}", a);
    println!("address pointer to value = {:?}", *a);
}