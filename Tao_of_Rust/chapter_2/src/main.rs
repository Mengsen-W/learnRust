/// Chapter 2 learning
/// mengsen mengsen_wang@163.com
/// 2020-11-8 22:25

fn main() {
    println!("{:?}", no_return_value());
    let a = &10;
    print_address(a);
    assert_eq!(math(sum, 1, 1), 2);
    assert_eq!(true_maker()(), true);
    let _arr = [0; init_len()];
    closure();
    let a = 2;
    let b = 3;
    assert_eq!(closure_math(|| a + b), 5);
    let result = two_times_impl();
    assert_eq!(result(2), 4);
    assert_eq!(while_true(5), 6);
    bind_mode();
    integer_limit();
    range_type();
    array_slice_type();
    string_type();
    raw_pointer();
    never_type_test();
    new_type_mode();
    unit_struct_type();
    enum_type();
    data_struct();
    multi_struct_single_trait();
    error_handle();
    smart_pointer();
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

fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn is_true() -> bool {
    true
}

fn true_maker() -> fn() -> bool {
    // return function pointer
    is_true
}

const fn init_len() -> usize {
    return 5;
}

fn closure() {
    let out = 3;
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    assert_eq!(5, closure_annotated(1, 1));
}

fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    // send a closure as a parameter
    op()
}

fn two_times_impl() -> impl Fn(i32) -> i32 {
    // send closure as a return value
    // bubble a warning, suggestion ues loop
    let i = 2;
    move |j| j * i
}

#[allow(while_true)]
fn while_true(x: i32) -> i32 {
    // flow sensitive analysis do not check condition of while
    // poop
    while true {
        return x + 1;
    }
    // so if do not this x
    // complier will think return value not a same type
    x
}

fn bind_mode() {
    let boolean = true;
    let mut binary = 0;
    // if let is a whole key word
    // if let boolean match true
    // binary = 1
    if let true = boolean {
        binary = 1;
    }
    assert_eq!(binary, 1);

    let mut v = vec![1, 2, 3, 4];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn integer_limit() {
    println!("{}", std::f32::INFINITY);
    println!("{}", std::f32::NEG_INFINITY);
    println!("{}", std::f32::NAN);
    println!("{}", std::f32::MAX);
    println!("{}", std::f32::MIN);
    println!("{}", std::i32::MAX);
    println!("{}", std::i32::MIN);
    println!("{}", std::u32::MAX);
    println!("{}", std::u32::MIN);
    println!("{}", std::usize::MAX);
    println!("{}", std::usize::MIN);
}

fn range_type() {
    for i in 1..5 {
        println!("hi number {}", i)
    }
    for i in 1..=5 {
        println!("hi number {}", i)
    }
}

fn array_slice_type() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], &[2, 3, 4, 5]);
    assert_eq!(&arr.len(), &5);
    assert_eq!(&arr.is_empty(), &false);
    // arr is slice type
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);
    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1, 2, 3]);
}

fn string_type() {
    let truth: &'static str = "Rust is a awesome";
    let ptr = truth.as_ptr();
    let len = truth.len();
    assert_eq!(17, len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));
}

fn raw_pointer() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &(*y) as *const i32;
    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);
}

// #![feature(never_type)]
fn never_type_test() -> i32 {
    // never type convert any type
    let num: Option<u32> = Some(42);
    match num {
        Some(num) => num,
        None => panic!("Nothing"),
    };

    // do not work
    // let x: ! = { return 123 };
    1
}

fn new_type_mode() {
    struct Integer(u32);
    type Int = i32;
    let int = Integer(10);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    assert_eq!(int, 10);
}

fn unit_struct_type() {
    // debug mode difference address
    // release mode same address
    // this mean all of Empty is the one in release mode
    struct Empty;
    let x = Empty;
    println!("{:p}", &x);
    let y = x;
    println!("{:p}", &y);
    let z = Empty;
    println!("{:p}", &z);
    assert_eq!((..), std::ops::RangeFull);
}

fn enum_type() {
    enum Number {
        Zero,
        One,
        Two,
    }
    let _a = Number::One;
    let _a = Number::Zero;
    let _a = Number::Two;
    match _a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    }

    enum Color {
        Red = 1,
        Green = 2,
        Blue = 3,
    }
    println!("color number = {}", Color::Red as i32);
    println!("color number = {}", Color::Green as i32);
    println!("color number = {}", Color::Blue as i32);

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    };

    let _x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
    let _y: fn(String) -> IpAddr = IpAddr::V6;

    let _home = IpAddr::V4(127, 0, 0, 1);
}

fn data_struct() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);
    let _v2 = vec![0; 10];
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);

    use std::collections::VecDeque;
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));
    buf.push_back(3);
    assert_eq!(buf.get(2), Some(&3));

    use std::collections::LinkedList;
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list1.append(&mut list2);
    println!("{:?}", list1);
    println!("{:?}", list2);
    list1.pop_front();
    println!("{:?}", list1);
    list1.push_front('e');
    println!("{:?}", list1);
    list1.pop_back();
    println!("{:?}", list1);

    use std::collections::BTreeMap;
    use std::collections::HashMap;
    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();
    hmap.insert(3, 'c');
    hmap.insert(1, 'a');
    hmap.insert(2, 'b');
    bmap.insert(3, 'c');
    bmap.insert(1, 'a');
    bmap.insert(2, 'b');
    println!("{:?}", hmap);
    println!("{:?}", bmap);

    use std::collections::BTreeSet;
    use std::collections::HashSet;
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();
    hbooks.insert('c');
    hbooks.insert('a');
    hbooks.insert('b');
    bbooks.insert('c');
    bbooks.insert('a');
    bbooks.insert('b');
    println!("{:?}", hbooks);
    println!("{:?}", bbooks);

    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
    let arr = [93, 80, 48];
    for &i in arr.iter() {
        heap.push(i);
    }
    assert_eq!(heap.peek(), Some(&93));
    println!("{:?}", heap);
}

fn multi_struct_single_trait() {
    struct Duck;
    struct Pig;
    trait Fly {
        fn fly(&self) -> bool {
            println!("default");
            return false;
        }
    }

    impl Fly for Duck {
        fn fly(&self) -> bool {
            return true;
        }
    }
    impl Fly for Pig {
        fn fly(&self) -> bool {
            return false;
        }
    }

    fn fly_static<T: Fly>(s: T) -> bool {
        s.fly()
    }
    fn fly_dyn(s: &dyn Fly) -> bool {
        s.fly()
    }

    let pig = Pig;
    assert_eq!(fly_static::<Pig>(pig), false);
    let duck = Duck;
    assert_eq!(fly_static::<Duck>(duck), true);
    assert_eq!(fly_dyn(&Pig), false);
    assert_eq!(fly_dyn(&Duck), true);
}

fn error_handle() {
    let x: Result<i32, &str> = Ok(3);
    assert_eq!(x.is_ok(), true);
    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
}

fn smart_pointer() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            (self.x == other.x) && (self.y == other.y)
        }
    }

    let boxed_point = Box::new(Point { x: 0.0, y: 0.0 });
    let unboxed_point: Point = *boxed_point;
    assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
}
