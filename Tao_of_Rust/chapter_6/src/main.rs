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
  {
    return_closure();
  }
  {
    advance_life_time();
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

fn return_closure() {
  {
    fn boxed_closure(c: &mut Vec<Box<dyn Fn()>>) {
      let s = "second";
      c.push(Box::new(|| println!("first")));
      c.push(Box::new(move || println!("{}", s)));
      c.push(Box::new(|| println!("third")));
    }
    let mut c: Vec<Box<dyn Fn()>> = vec![];
    boxed_closure(&mut c);
    for f in c {
      f();
    }
  }
  {
    trait Any {
      fn any<F>(&self, f: F) -> bool
      where
        Self: Sized,
        F: Fn(u32) -> bool;
    }
    impl Any for Vec<u32> {
      fn any<F>(&self, f: F) -> bool
      where
        Self: Sized,
        F: Fn(u32) -> bool,
      {
        for &x in self {
          if f(x) {
            return true;
          }
        }
        false
      }
    }
    let v = vec![1, 2, 3];
    let b = v.any(|x| x == 3);
    println!("{:?}", b);
  }
  {
    trait Any {
      fn any(&self, f: &(dyn Fn(u32) -> bool)) -> bool;
    }
    impl Any for Vec<u32> {
      fn any(&self, f: &(dyn Fn(u32) -> bool)) -> bool {
        for &x in self {
          if f(x) {
            return true;
          }
        }
        false
      }
    }
    let v = vec![1, 2, 3];
    let b = v.any(&|x| x == 3);
    println!("{:?}", b);
  }
  {
    fn call<F>(closure: F) -> i32
    where
      F: Fn(i32) -> i32,
    {
      closure(1)
    }
    fn counter(i: i32) -> i32 {
      i + 1
    }
    let result = call(counter);
    assert_eq!(2, result);
  }
  {
    fn square() -> Box<dyn Fn(i32) -> i32> {
      Box::new(|i| i * i)
    }
    let square = square();
    assert_eq!(4, square(2));
    assert_eq!(9, square(3));
  }
  {
    fn square() -> impl FnOnce(i32) -> i32 {
      |i| i * i
    }
    let square = square();
    assert_eq!(4, square(2));
  }
}

fn advance_life_time() {
  {
    use std::fmt::Debug;
    trait DoSomething<T> {
      fn do_sth(&self, value: T);
    }

    impl<'a, T: Debug> DoSomething<T> for &'a usize {
      fn do_sth(&self, value: T) {
        println!("{:?}", value);
      }
    }

    fn foo(b: Box<dyn for<'f> DoSomething<&'f usize>>) {
      let s: usize = 10;
      // s must longer than foo(x)
      b.do_sth(&s);
    }
    let x = Box::new(&2usize);
    foo(x);
  }
  {
    struct Pick<F> {
      data: (u32, u32),
      func: F,
    }
    impl<F> Pick<F>
    where
      F: Fn(&(u32, u32)) -> &u32,
    {
      fn call(&self) -> &u32 {
        (self.func)(&self.data)
      }
    }
    fn max(data: &(u32, u32)) -> &u32 {
      if data.0 > data.1 {
        &data.0
      } else {
        &data.1
      }
    }
    let elm = Pick {
      data: (3, 1),
      func: max,
    };
    println!("{}", elm.call());
  }
}

fn iterator() {
  {
    // internal iterator
    trait InIterator<T: Copy> {
      fn each<F: Fn(T) -> T>(&mut self, f: F);
    }
    impl<T: Copy> InIterator<T> for Vec<T> {
      fn each<F: Fn(T) -> T>(&mut self, f: F) {
        let mut i = 0;
        while i < self.len() {
          self[i] = f(self[i]);
          i += 1;
        }
      }
    }

    let mut v = vec![1, 2, 3];
    v.each(|i| i * 3);
    assert_eq!([3, 6, 9], v[..]);
  }
  {
    let v = vec![1, 2, 3];
    for i in v {
      println!("{:?}", i);
    }
  }
  {
    let v = vec![1, 2, 3];
    let mut _it = v.into_iter();
    loop {
      match _it.next() {
        Some(i) => {
          println!("{}", i);
        }
        None => break,
      }
    }
  }
  {
    trait Iterator {
      type Item;
      fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
      count: usize,
    }
    impl Iterator for Counter {
      type Item = usize;
      fn next(&mut self) -> Option<usize> {
        self.count += 1;
        if self.count < 3 {
          Some(self.count)
        } else {
          None
        }
      }
    }
    let mut counter = Counter { count: 0 };
    assert_eq!(Some(1), counter.next());
    assert_eq!(Some(2), counter.next());
    assert_eq!(Some(3), counter.next());
    assert_eq!(None, counter.next());
  }
  {
    let a: [i32; 3] = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());
    iter.next();
    assert_eq!((2, Some(2)), iter.size_hint());
  }
  {
    let arr1 = [1, 2, 3, 4, 5];
    let c1 = arr1.iter().map(|x| x * 2).collect::<Vec<i32>>();
    assert_eq!(&c1[..], [2, 4, 6, 8, 10]);
    let arr2 = ["1", "2", "3", "h"];
    let c2 = arr2
      .iter()
      .filter_map(|x| x.parse().ok())
      .collect::<Vec<i32>>();
    assert_eq!(&c2[..], [1, 2, 3]);
    let arr3 = ['a', 'b', 'c'];
    for (idx, val) in arr3.iter().enumerate() {
      println!("idx: {:?}, val: {:?}", idx, val.to_uppercase());
    }

    let a = [1, 2, 3];
    let mut iter = a.iter().rev();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));

    let nums = vec![1, 2, 3];
    let mut iter = nums.into_iter();
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(3), iter.next_back());
    assert_eq!(Some(2), iter.next_back());
    assert_eq!(None, iter.next());
  }
  {
    let a = [1, 2, 3];
    assert_eq!(a.iter().any(|&x| x != 2), true);
    assert_eq!(a.iter().any(|x| *x != 2), true);
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);

    let arr = vec![1, 2, 3];
    let suml = arr.iter().fold(0, |acc, x| acc + x);
    let sum2 = arr.iter().fold(0, |acc, x| acc + *x);
    let sum3 = arr.iter().fold(0, |acc, &x| acc + x);
    let sum4 = arr.into_iter().fold(0, |acc, x| acc + x);
    assert_eq!(suml, 6);
    assert_eq!(sum2, 6);
    assert_eq!(sum3, 6);
    assert_eq!(sum4, 6);
  }
}
