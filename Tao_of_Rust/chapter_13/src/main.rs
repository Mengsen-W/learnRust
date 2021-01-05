#![feature(untagged_unions)]

fn main() {
    {
        unsafe_learning();
    }
}

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
        let hello = unsafe {
            String::from_utf8_unchecked(hello)
        };
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
        let _int_0 = MyZero{i: Value{tag: b'0', value: U {i: 0} } };
        let _float_0 = MyZero{i: Value{tag: b'1', value: U {f: 0.0} } };
    }
    {
        #[repr(u32)]
        enum Tag { I, F }
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

        fn is_zero(v: Value) -> bool {
            unsafe {
                match v {
                    Value { tag: Tag::I, u: U { i: 0} } => true,
                    Value { tag: Tag::F, u: U { f: 0. } } => true,
                    _ => false,
                }
            }
        }

        let int_0 = Value{tag: Tag::I, u: U{ i: 0 } };
        let float_0 = Value{tag: Tag::F, u: U {f: 0.0} };
        assert_eq!(true, is_zero(int_0));
        assert_eq!(true, is_zero(float_0));
        assert_eq!(4, std::mem::size_of::<U>());

    }
}
