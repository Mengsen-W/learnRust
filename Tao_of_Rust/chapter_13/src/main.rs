#![feature(untagged_unions)]
#![feature(allocator_api)]
#![feature(dropck_eyepatch)]

extern crate jemallocator;
use jemallocator::Jemalloc;

fn main() {
    {
        unsafe_learning();
    }
}

#[allow(unused_variables, unused_assignments, dead_code)]
fn unsafe_learning() {
    {
        let mut a = "hello";
        let b = &a;
        assert_eq!("hello", *b);
        let c = &mut a;
        *c = "helloWorld";
        assert_eq!("helloWorld", *c);
    }
    {
        let hello = vec![104, 101, 108, 108, 111];
        let hello = unsafe { String::from_utf8_unchecked(hello) };
        assert_eq!("hello", hello);
    }
    {
        let inc = 3;
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += inc;
            assert_eq!(3, COUNTER);
        }
    }
    {
        #[repr(C)]
        union U {
            i: i32,
            f: f32,
        }
        #[repr(C)]
        struct Value {
            tag: u8,
            value: U,
        }
        #[repr(C)]
        union MyZero {
            i: Value,
            f: Value,
        }
        #[repr(C)]
        enum _MyEnumZero {
            I(i32),
            F(f32),
        }
        let _int_0 = MyZero {
            i: Value {
                tag: b'0',
                value: U { i: 0 },
            },
        };
        let _float_0 = MyZero {
            i: Value {
                tag: b'1',
                value: U { f: 0.0 },
            },
        };
    }
    {
        #[repr(u32)]
        enum Tag {
            I,
            F,
        }
        #[repr(C)]
        union U {
            i: i32,
            f: f32,
        }
        #[repr(C)]
        struct Value {
            tag: Tag,
            u: U,
        }

        #[allow(illegal_floating_point_literal_pattern)]
        fn is_zero(v: Value) -> bool {
            unsafe {
                match v {
                    Value {
                        tag: Tag::I,
                        u: U { i: 0 },
                    } => true,
                    Value {
                        tag: Tag::F,
                        u: U { f: 0. },
                    } => true,
                    _ => false,
                }
            }
        }

        let int_0 = Value {
            tag: Tag::I,
            u: U { i: 0 },
        };
        let float_0 = Value {
            tag: Tag::F,
            u: U { f: 0.0 },
        };
        assert_eq!(true, is_zero(int_0));
        assert_eq!(true, is_zero(float_0));
        assert_eq!(4, std::mem::size_of::<U>());
    }
    {
        let mut s = "hello".to_string();
        let r1 = &s as *const String;
        let r2 = &mut s as *mut String;
        assert_eq!(r1, r2);
        let address: i64 = 0x7fff1d72307d;
        let _r3 = address as *const String;
        unsafe {
            assert_eq!("hello", *r1);
            assert_eq!("hello", *r2);
            // segmentation fault
            // println!("{}", *r3)
        }
        {
            let p: *const u8 = std::ptr::null();
            assert!(p.is_null());
            let s: &str = "hello";
            let ptr: *const u8 = s.as_ptr();
            assert!(!ptr.is_null());
            let mut s = [1, 2, 3];
            let ptr: *mut u32 = s.as_mut_ptr();
            assert!(!ptr.is_null());

            let s: &str = "Rust";
            let ptr: *const u8 = s.as_ptr();
            unsafe {
                assert_eq!('u', *ptr.offset(1) as char);
                assert_eq!('t', *ptr.offset(3) as char);
                // undefined
                // println!("{:?}",*ptr.offset(2557) as char);
            }

            let x = "hello".to_string();
            let y: *const u8 = x.as_ptr();
            unsafe {
                assert_eq!(y.read() as char, 'h');
            }

            let x = [0, 1, 2, 3];
            let y = x[0..].as_ptr() as *const [u32; 4];
            unsafe {
                assert_eq!(y.read(), [0, 1, 2, 3]);
            }

            let mut x = "";
            let y = &mut x as *mut &str;
            let z = "hello";
            unsafe {
                y.write(z);
                assert_eq!(y.read(), "hello");
            }

            let mut v: Vec<i32> = vec![1, 2];
            let v_ptr: *mut i32 = v.as_mut_ptr();
            unsafe {
                let old_v = v_ptr.replace(5);
                assert_eq!(1, old_v);
                assert_eq!([5, 2], &v[..])
            }

            let mut v: Vec<i32> = vec![1, 2];
            let v_ptr = &mut v as *mut Vec<i32>;
            unsafe {
                let old_v = v_ptr.replace(vec![3, 4, 5]);
                assert_eq!([1, 2], &old_v[..]);
                assert_eq!([3, 4, 5], &v[..])
            }

            let mut array = [0, 1, 2, 3];
            let x = array[0..].as_mut_ptr() as *mut [u32; 2];
            let y = array[1..].as_mut_ptr() as *mut [u32; 2];
            unsafe {
                assert_eq!([0, 1], x.read());
                assert_eq!([1, 2], y.read());
                x.swap(y);
                assert_eq!([1, 0, 1, 3], array);
            }
        }
    }
    {
        {
            struct MyCell<T> {
                value: T,
            }
            #[allow(dead_code)]
            impl<T: Copy> MyCell<T> {
                fn new(x: T) -> MyCell<T> {
                    MyCell { value: x }
                }
                fn get(&self) -> T {
                    self.value
                }
                fn set(&self, value: T) {
                    use std::ptr;
                    unsafe {
                        ptr::write(&self.value as *const _ as *mut _, value);
                    }
                }
            }

            fn step1<'a>(r_cl: &MyCell<&'a i32>) {
                let val: i32 = 13;
                step2(&val, r_cl);
                assert_eq!(13, *r_cl.value);
            }
            fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) {
                r_c2.set(r_val);
            }

            static X: i32 = 10;
            let cell = MyCell::new(&X);
            step1(&cell);
            // undefined
            // assert_eq!(0, *cell.value);
        }
        {
            use std::marker::PhantomData;
            #[allow(dead_code)]
            struct MyCell<T> {
                value: T,
                mark: PhantomData<fn(T)>,
            }
            #[allow(dead_code)]
            impl<T: Copy> MyCell<T> {
                fn new(x: T) -> MyCell<T> {
                    MyCell {
                        value: x,
                        mark: PhantomData,
                    }
                }
                fn get(&self) -> T {
                    self.value
                }
                fn set(&self, value: T) {
                    use std::ptr;
                    unsafe {
                        ptr::write(&self.value as *const _ as *mut _, value);
                    }
                }
            }
        }
        {
            fn foo(input: &str) {
                format! {"{:?}", input};
            }
            fn bar(f: fn(&'static str), v: &'static str) {
                (f)(v);
            }

            let v: &'static str = "hello";
            bar(foo, v);
        }
    }
    {
        {
            fn foo<'a>(input: *const u32) -> &'a u32 {
                unsafe {
                    return &*input;
                }
            }

            let x;
            {
                let y = 42;
                x = foo(&y);
            }
            assert_eq!(*x, 42);
        }
        {
            use std::mem::transmute;
            let x: &i32;
            {
                let a = 12;
                let ptr = &a as *const i32;
                x = unsafe { transmute::<*const i32, &i32>(ptr) };
                assert_eq!(12, *x);
            }
        }
    }
    {
        {
            use std::fmt;
            #[derive(Copy, Clone, Debug)]
            enum State {
                InValid,
                Valid,
            }
            #[derive(Debug)]
            struct Hello<T: fmt::Debug>(&'static str, T, State);
            impl<T: fmt::Debug> Hello<T> {
                fn new(name: &'static str, t: T) -> Self {
                    Hello(name, t, State::Valid)
                }
            }
            impl<T: fmt::Debug> Drop for Hello<T> {
                fn drop(&mut self) {
                    println!("drop Hello ({}, {:?}, {:?})", self.0, self.1, self.2);
                    self.2 = State::InValid;
                }
            }

            struct WrapBox<T> {
                v: Box<T>,
            }
            impl<T> WrapBox<T> {
                fn new(t: T) -> Self {
                    WrapBox { v: Box::new(t) }
                }
            }
            fn f1() {
                // live error
                // let (x, y);
                let x;
                let y;
                x = Hello::new("x", 13);
                y = WrapBox::new(Hello::new("y", &x));
            }
            f1();
        }
        {
            use std::alloc::{GlobalAlloc, Layout, System};
            use std::fmt;
            use std::mem;
            use std::ptr;
            #[derive(Copy, Clone, Debug)]
            enum State {
                InValid,
                Valid,
            }
            #[derive(Debug)]
            struct Hello<T: fmt::Debug>(&'static str, T, State);
            impl<T: fmt::Debug> Hello<T> {
                fn new(name: &'static str, t: T) -> Self {
                    Hello(name, t, State::Valid)
                }
            }
            impl<T: fmt::Debug> Drop for Hello<T> {
                fn drop(&mut self) {
                    println!("drop Hello ({}, {:?}, {:?})", self.0, self.1, self.2);
                    self.2 = State::InValid;
                }
            }

            struct MyBox<T> {
                v: *const T,
            }

            impl<T> MyBox<T> {
                fn new(t: T) -> Self {
                    unsafe {
                        let p = System.alloc(Layout::array::<T>(1).unwrap());
                        let p = p as *mut T;
                        ptr::write(p, t);
                        MyBox { v: p }
                    }
                }
            }

            unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
                fn drop(&mut self) {
                    unsafe {
                        ptr::read(self.v);
                        let p = self.v as *mut _;
                        System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
                    }
                }
            }

            struct WrapBox<T> {
                v: Box<T>,
            }
            impl<T> WrapBox<T> {
                fn new(t: T) -> Self {
                    WrapBox { v: Box::new(t) }
                }
            }

            fn f2() {
                {
                    let (x, y);
                    x = Hello::new("x", 13);
                    y = WrapBox::new(Hello::new("y", &x));
                }
                {
                    let (y2, x2);
                    x2 = Hello::new("x2", 13);
                    y2 = MyBox::new(Hello::new("y2", &x2));
                }
            }
            f2();
        }
        {
            use std::alloc::{GlobalAlloc, Layout, System};
            use std::fmt;
            use std::marker::PhantomData;
            use std::mem;
            use std::ptr;

            #[derive(Copy, Clone, Debug)]
            enum State {
                InValid,
                Valid,
            }
            #[derive(Debug)]
            struct Hello<T: fmt::Debug>(&'static str, T, State);
            impl<T: fmt::Debug> Hello<T> {
                fn new(name: &'static str, t: T) -> Self {
                    Hello(name, t, State::Valid)
                }
            }
            impl<T: fmt::Debug> Drop for Hello<T> {
                fn drop(&mut self) {
                    println!("drop Hello ({}, {:?}, {:?})", self.0, self.1, self.2);
                    self.2 = State::InValid;
                }
            }

            struct MyBox<T> {
                v: *const T,
                _pd: PhantomData<T>,
            }
            impl<T> MyBox<T> {
                fn new(t: T) -> Self {
                    unsafe {
                        let p = System.alloc(Layout::array::<T>(1).unwrap());
                        let p = p as *mut T;
                        ptr::write(p, t);
                        MyBox {
                            v: p,
                            _pd: Default::default(),
                        }
                    }
                }
            }
            unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
                fn drop(&mut self) {
                    unsafe {
                        ptr::read(self.v);
                        let p = self.v as *mut _;
                        System.dealloc(p, Layout::array::<T>(mem::align_of::<T>()).unwrap());
                    }
                }
            }
            fn f3() {
                let x;
                let y;
                x = Hello::new("x", 13);
                y = MyBox::new(Hello::new("y", &x));
            }
            f3();
        }
        {
            use std::mem;
            struct A;
            struct B;
            struct Foo {
                a: A,
                b: B,
            }
            impl Foo {
                fn take(mut self) -> (A, B) {
                    let a = mem::replace(&mut self.a, unsafe {
                        mem::MaybeUninit::zeroed().assume_init()
                    });
                    let b = mem::replace(&mut self.b, unsafe {
                        mem::MaybeUninit::zeroed().assume_init()
                    });
                    mem::forget(self);
                    (a, b)
                }
            }
        }
        {
            use std::mem::ManuallyDrop;
            struct Peach;
            struct Banana;
            struct Melon;
            struct FruitBox {
                peach: ManuallyDrop<Peach>,
                melon: Melon,
                banana: ManuallyDrop<Banana>,
            }
            impl Drop for FruitBox {
                fn drop(&mut self) {
                    unsafe {
                        ManuallyDrop::drop(&mut self.peach);
                        ManuallyDrop::drop(&mut self.banana);
                    }
                }
            }
        }
    }
    {
        {
            use std::ptr::{null, NonNull};
            let ptr: NonNull<i32> = NonNull::dangling();
            println!("{:?}", ptr);
            let mut v = 42;
            let ptr: Option<NonNull<i32>> = NonNull::new(&mut v);
            println!("{:?}", ptr);
            println!("{:?}", ptr.unwrap().as_ptr());
            println!("{:?}", unsafe { ptr.unwrap().as_mut() });
            let mut v = 42;
            let ptr = NonNull::from(&mut v);
            println!("{:?}", ptr);
            let null_p: *const i32 = null();
            let ptr = NonNull::new(null_p as *mut i32);
            println!("{:?}", ptr);
        }
        {
            use std::mem;
            use std::ptr::NonNull;
            struct Foo {
                a: *mut u64,
                b: *mut u64,
            }
            struct FooUsingNonNull {
                a: *mut u64,
                b: NonNull<*mut u64>,
            }
            println!("*mut u64: {} bytes", mem::size_of::<*mut u64>());
            println!(
                "NonNull<*mut u64>: {} bytes",
                mem::size_of::<NonNull<*mut u64>>()
            );
            println!(
                "Option<*mut u64>: {} bytes",
                mem::size_of::<Option<*mut u64>>()
            );
            println!(
                "Option<NonNull<*mut u64>>: {} bytes",
                mem::size_of::<Option<NonNull<*mut u64>>>()
            );
            println!("Option<Foo>: {} bytes", mem::size_of::<Option<Foo>>());
            println!(
                "Option<FooUsingNonNull>: {} bytes",
                mem::size_of::<Option<FooUsingNonNull>>()
            );
        }
        {
            //            impl<T: Clone> Vec<T> {
            //                fn push_all(&mut self, to_push: &[T]) {
            //                    self.reserve(to_push.len());
            //                    unsafe {
            //                        self.set_len(self.len() + to_push.len());
            //                        for (i, x) in to_push.iter().enumerate() {
            //                            self.ptr().offset(i as isize).write(x.clone());
            //                        }
            //                    }
            //                }
            //            }
        }
        {
            use std::alloc::{GlobalAlloc, Layout, System};
            struct MyAllocator;
            unsafe impl GlobalAlloc for MyAllocator {
                unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
                    System.alloc(layout)
                }
                unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
                    System.dealloc(ptr, layout)
                }
            }
            //            #[global_allocator]
            //            static GLOBAL: MyAllocator = MyAllocator;
            let mut v = Vec::new();
            v.push(1);
        }
        {
            #[global_allocator]
            static GLOBAL: Jemalloc = Jemalloc;
        }
    }
}
