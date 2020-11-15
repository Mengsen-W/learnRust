/// chapter 6 functional closure and iterator
/// mengsen
/// 2020-11-14 23:10

fn main() {
    {
        assert_eq!(r#match("a", "ab"), true);
    }
    {
        let v = vec![1];
        // ownership move to function mutable variable
        // v converse mut v
        let v = modify_move_ownership(v);
        println!("{:?}", v);
    }
    {
        let mut v = vec![1, 2];
        // &mut v type is v: &mut [i32]
        // function type is &mut[i32]
        // mutable ownership is just one
        // do not move ownership
        modify_references(&mut v);
        println!("{:?}", v);
    }
    {
        variable_shadow();
    }
    {
        parameter_bind();
    }
}

fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn modify_move_ownership(mut v: Vec<u32>) -> Vec<u32> {
    v.push(100);
    v
}

fn modify_references(v: &mut [u32]) {
    v.reverse();
}

#[allow(dead_code)]
fn f() {
    println!("1");
}

fn variable_shadow() {
    f();
    {
        f();
        fn f() {
            println!("2");
        }
    }
    f();
    fn f() {
        println!("3");
    }
}

fn parameter_bind() {
    #[derive(Debug)]
    struct S {
        i: i32,
    }
    fn f_ref(ref _s: S) {
        println!("{:p}", _s);
        println!("{:?}", _s);
    }

    fn f(ref _s: &S) {
        println!("{:p}", _s);
        println!("{:?}", *_s);
    }

    fn foo(_: i32) {
        println!("no param");
    }

    let s = S { i: 42 };
    f(&s);
    f_ref(s);
    foo(3);
}
