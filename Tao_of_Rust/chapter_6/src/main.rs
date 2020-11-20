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
  {
    closure();
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

// fn closure_principle() {
//     #![feature(unboxed_closures, fn_traits)]
//     struct Closure {
//         env_var: u32,
//     }
//     impl FnOnce<()> for Closure {
//         type Output = u32;
//         extern "rus-call" fn call_once(self, args: ()) -> u32 {
//             println!("call it FnOnce()");
//             self.env_var + 2
//         }
//     }
//     impl FnMut<()> for Closure {
//         extern "rust-call" fn call_mut(&mut self, args: ()) -> u32 {
//             println!("call it FnMut()");
//             self.env_var + 2
//         }
//     }
//     impl Fn<()> for Closure {
//         extern "rust-call" fn call(&self, args: ()) -> u32 {
//             println!("call it FnMut()");
//             self.env_var + 2
//         }
//     }

//     fn call_it<F: Fn() -> u32>(f: &F) -> u32 {
//         f()
//     }
//     fn call_it_mut<F: FnMut() -> u32>(f: &mut F) -> u32 {
//         f()
//     }
//     fn call_it_once<F: FnOnce() -> u32>(f: F) -> u32 {
//         f()
//     }
// }

fn closure() {
  {
    // Fn trait
    let s = "hello";
    let c = || println!("{:?}", s);
    c();
    c();
    println!("{:?}", s);
  }
  {
    // FnOnce trait
    {
      let s = "hello".to_string();
      let c = || s;
      c();
      // c();
    }
    {
      let s = "hello".to_string();
      let c = move || s;
      c();
      // c();
    }
  }
  {
    // move copy
    let s = "hello";
    let c = move || println!("{:?}", s);
    c();
    c();
    println!("{:?}", s);
  }
  {
    // move non-copy
    let s = "hello".to_string();
    let c = move || println!("{:?}", s);
    c();
    c();
    // println!("{:?}", s);
  }
  #[allow(unused_variables)]
  {
    // this is FnOnce if no move this just moved
    // if has move this copy that has copy trait
    fn call<F: FnOnce()>(f: F) {
      f()
    }
    let mut x = 0;
    // move make x copy because x has copy/clone
    // in seconds x is moved
    let incr_x = || x += 1;
    call(incr_x);
    // call(incr_x);
    assert_eq!(x, 1);

    // every call function is moved
    let mut incr_x = move || x += 1;
    call(incr_x);
    incr_x();
    call(incr_x);
    assert_eq!(x, 1);

    // do not copy move
    let mut x = vec![];
    let expend_x = move || x.push(1);
    // do not move make x because x has not copy/clone
    call(expend_x);
  }

  // FnMut trait
  let mut s = "rush".to_string();
  {
    let mut c = || s += " rust";
    c();
    c();
    println!("{:?}", s);
  }
  println!("{:?}", s);
  {
    let c = || println!("{:?}", "rust");
    c();
    c();
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
