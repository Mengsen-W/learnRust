/// chapter 3 type system
/// mengsen
/// 2020-11-10 15:03

fn main() {
    {
        fat_pointer();
    }

    {
        let mut arr = [1, 2, 3];
        reset_1(arr);
        reset_2(&mut arr);
        assert_eq!(std::mem::size_of::<&mut [u32; 3]>(), 8);
        assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);
    }

    {
        zero_sized_type();
    }

    {
        turbo_fish();
    }

    {
        generic_type();
    }

    {
        interface_abstract();
    }
}

fn fat_pointer() {
    let str = "hello rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:?}", ptr);
    println!("{:?}", len);
}

fn reset_1(mut arr: [u32; 3]) {
    arr[0] = 3;
    arr[1] = 2;
    arr[2] = 1;
    println!("{:?}", arr);
}

fn reset_2(arr: &mut [u32]) {
    arr[0] = 3;
    arr[1] = 2;
    arr[2] = 1;
    println!("{:?}", arr);
}

fn zero_sized_type() {
    enum Void {};
    struct Foo {};
    #[allow(dead_code)]
    struct Baz {
        foo: Foo,
        qux: (),
        // zero u8 in array
        baz: [u8; 0],
    };

    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<Foo>(), 0);
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0);

    let v: Vec<()> = vec![(); 3];
    // Vec optimized ()
    for i in v {
        println!("{:?}", i);
    }
}

fn turbo_fish() {
    let x = "1";
    let int_x: i32 = x.parse().unwrap();
    assert_eq!(int_x, 1);

    let x = "2";
    // ::<> like a fish
    let int_x = x.parse::<i32>().unwrap();
    assert_eq!(int_x, 2);
}

fn generic_type() {
    fn foo<T>(x: T) -> T {
        return x;
    }
    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");

    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    };
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
    }
    let point1 = Point::new(1, 2);
    let point2 = Point::new("1", "2");
    assert_eq!(point1, Point { x: 1, y: 2 });
    assert_eq!(point2, Point { x: "1", y: "2" });

    #[derive(Debug, PartialEq)]
    struct Foo(i32);
    #[derive(Debug, PartialEq)]
    struct Bar(i32, i32);
    trait Inst {
        fn new(i: i32) -> Self;
    }
    impl Inst for Foo {
        fn new(i: i32) -> Foo {
            Foo(i)
        }
    }
    impl Inst for Bar {
        fn new(i: i32) -> Bar {
            Bar(i, i + 10)
        }
    }
    fn foobar<T: Inst>(i: i32) -> T {
        T::new(i)
    }
    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));
    let b: Bar = foobar(20);
    assert_eq!(b, Bar(20, 30));
}

fn interface_abstract() {
    {
        // RHS mean right side of '+'
        // Output mean left side of '='
        trait AddMy<RHS, Output> {
            fn my_add(self, rhs: RHS) -> Output;
        }
        impl AddMy<i32, i32> for i32 {
            fn my_add(self, rhs: i32) -> i32 {
                self + rhs
            }
        }
        impl AddMy<u32, i32> for u32 {
            fn my_add(self, rhs: u32) -> i32 {
                (self + rhs) as i32
            }
        }
        let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
        let x: i32 = a.my_add(b);
        let y: i32 = c.my_add(d);
        assert_eq!(x, 3i32);
        assert_eq!(y, 7i32);
    }

    {
        // just one generic parameter
        // RHS default is Self
        trait AddStd<RHS = Self> {
            type Output;
            fn my_add_std(self, rhs: RHS) -> Self::Output;
        }
        // use default RHS
        impl AddStd for u32 {
            type Output = u32;
            fn my_add_std(self, other: u32) -> u32 {
                self + other
            }
        }
        // use set RHS
        impl AddStd<&str> for String {
            type Output = String;
            fn my_add_std(mut self, other: &str) -> String {
                self.push_str(other);
                self
            }
        }
        let a = String::from("hello");
        let b = "world";
        let c = a.my_add_std(b);
        println!("{:?}", c);
    }

    {
        trait Add<RHS = Self> {
            type Output;
            fn add(self, rhs: RHS) -> Self::Output;
        }
        impl Add<u64> for u32 {
            type Output = u64;
            fn add(self, other: u64) -> Self::Output {
                (self as u64) + other
            }
        }
        let a = 1u32;
        let b = 2u64;
        assert_eq!(a.add(b), 3);
    }

    {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        use std::ops::Add;
        impl Add for Point {
            type Output = Point;
            fn add(self, other: Point) -> Self::Output {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
        println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
    }

    {
        trait Page {
            fn set_page(&self, _p: i32) {
                println!("Page Default: 1");
            }
        }
        trait PerPage {
            fn set_perpage(&self, _num: i32) {
                println!("Page Default: 10");
            }
        }
        // inheritance trait
        trait Paginate: Page + PerPage {
            fn set_skip_page(&self, num: i32) {
                println!("Skip Page: {:?}", num);
            }
        }
        struct MyPaginate {
            _page: i32,
        }
        // default implementation
        impl Page for MyPaginate {}
        impl PerPage for MyPaginate {}
        // Implement Paginate for all types
        // that have Page and PerPage behavior.
        impl<T: Page + PerPage> Paginate for T {}

        let my_paginate = MyPaginate { _page: 1 };
        my_paginate.set_page(1);
        my_paginate.set_perpage(100);
        my_paginate.set_skip_page(12);
    }
}
