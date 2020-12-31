use std::any::{Any, TypeId};
#[derive(Debug)]
enum E {
    H,
    He,
    Li,
}
struct S {
    x: u8,
    y: u8,
    z: u16,
}

fn main() {
    {
        any_learning();
    }
    {
        macro_learning();
    }
}

fn any_learning() {
    {
        let v1 = 0xc0ffee_u32;
        let v2 = E::He;
        let v3 = S {
            x: 0xde,
            y: 0xad,
            z: 0xbeef,
        };
        let v4 = "rust";
        let mut a: &dyn Any;
        a = &v1;
        assert!(a.is::<u32>());
        println!("{:?}", TypeId::of::<u32>());
        a = &v2;
        assert!(a.is::<E>());
        println!("{:?}", TypeId::of::<E>());
        a = &v3;
        assert!(a.is::<S>());
        println!("{:?}", TypeId::of::<S>());
        a = &v4;
        assert!(a.is::<&str>());
        println!("{:?}", TypeId::of::<&str>());
    }
    {
        fn print_any(a: &dyn Any) {
            if let Some(v) = a.downcast_ref::<u32>() {
                println!("u32 {:x}", v);
            } else if let Some(v) = a.downcast_ref::<E>() {
                println!("enum E {:?}", v);
            } else if let Some(v) = a.downcast_ref::<S>() {
                println!("struct S {:x} {:x} {:x}", v.x, v.y, v.z);
            } else {
                println!("else");
            }
        }

        print_any(&0xc0ffee_u32);
        print_any(&E::He);
        print_any(&S {
            x: 0xde,
            y: 0xad,
            z: 0xbeef,
        });
        print_any(&"rust");
        print_any(&"hoge");
    }
    {
        fn print_if_string(value: Box<dyn Any>) {
            if let Ok(string) = value.downcast::<String>() {
                println!("String (lentgh {}): {}", string.len(), string);
            } else {
                println!("Not String");
            }
        }
        let my_string = "Hello World".to_string();
        print_if_string(Box::new(my_string));
        print_if_string(Box::new(0i8));
    }
    {
        struct UnStatic<'a> {
            x: &'a i32,
        }
        static ANSWER: i32 = 42;
        let v = UnStatic { x: &ANSWER };
        let mut a: &dyn Any;
        a = &v;
        assert!(a.is::<UnStatic>());
    }
}

fn macro_learning() {
    {
        macro_rules! unless {
            ($arg:expr, $branch:expr) => {
                if !$arg {
                    $branch
                };
            };
        }
        fn cmp(a: i32, b: i32) {
            unless!(a > b, {
                println!("{} < {}", a, b);
            });
        }
        let (a, b) = (1, 2);
        cmp(a, b);
    }
    {
        macro_rules! hashmap {
        ($($key: expr => $value: expr), * $(,)*) => {
          {
            let mut _map = ::std::collections::HashMap::new();
          $(_map.insert($key, $value);)*
          _map
          };
        }
      }

        let map = hashmap! {
          "a" => 1,
          "b" => 2,
        };
        assert_eq!(map["a"], 1);
    }

    {
        // macro_rules! unit {
        //   ($($x:tt)*) => (());
        // }
        // macro_rules! count {
        //   ($($key:expr), *) =>(<[()]>::len(&[$(unit!($key)),*]));
        // }
        macro_rules! hashmap {
        (@unit $($x:tt)*) => (());
        (@count $($rest:expr), *) =>
          (<[()]>::len(&[$(hashmap!(@unit! $rest)),*]));
        ($($key: expr => $value: expr), * $(,)*) => {
          {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
          $(_map.insert($key, $value);)*
          _map
          };
        }
      }
        let map = hashmap! {
          "a" => 1,
          "b" => 2,
        };
        assert_eq!(map["a"], 1);
    }
}
