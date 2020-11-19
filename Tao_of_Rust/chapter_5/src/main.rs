/// chapter 5 ownership
/// mengsen
/// 2020-11-13 11:46:07

fn main() {
  {
    ownership();
  }
  {
    life_time();
  }
  {
    let mut a = vec![1, 3, 7, 4, 2];
    bubble_sort(&mut a);
    println!("{:?}", a)
  }
  {
    let i = 20;
    let mut o = 5;
    brrow_check(&i, &mut o);
  }
  {
    assert_eq!("hello", the_longest("hello", "rust"));
  }
  {
    struct_life_time();
  }
  {
    assert_eq!("hello", life_time_note("hello", "rust"));
  }
  {
    life_limit();
  }
  {
    smart_pointer();
  }
  {
    rc_and_weak();
  }
  {
    cell_and_ref_cell();
  }
  {
    copy_on_write();
  }
  {
    token();
  }
  {
    none_lexical_lifetime();
  }
}

#[allow(unused_variables)]
fn ownership() {
  #[derive(Debug, Copy, Clone)]
  // add Copy derive
  struct A {
    a: i32,
    b: i32,
  }
  let a = A { a: 1, b: 2 };
  let b = a;
  println!("{:?}", a);

  let a = ("a".to_string(), "b".to_string());
  let b = a;
  // println!("{:?}", a); // String do not copy trait
  let c = (1, 2, 3);
  let d = c;
  println!("{:?}", c);
}

#[allow(unused_variables)]
fn life_time() {
  #[derive(Debug)]
  struct A {
    data: String,
  }

  impl Drop for A {
    fn drop(&mut self) {
      println!("dropping {:?}", self);
    }
  }

  let a = A {
    data: "hello".to_string(),
  };
  let b = A {
    data: "world".to_string(),
  };
  let c = A {
    data: "rust".to_string(),
  };
  // reference copy
  let d = c;
}

fn bubble_sort(a: &mut Vec<i32>) {
  let mut n = a.len();
  while n > 0 {
    let (mut i, mut max_ptr) = (1, 0);
    while i < n {
      if a[i - 1] > a[i] {
        a.swap(i - 1, i);
        max_ptr = i;
      }
      i += 1;
    }
    n = max_ptr
  }
}

fn brrow_check(input: &u32, output: &mut u32) {
  // complier optimized
  let cached_input = *input;
  if cached_input > 10 {
    *output = 2;
  } else if cached_input > 5 {
    *output *= 2;
  }
}

fn life_time_note<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  if s1.len() > s2.len() {
    s1
  } else {
    s2
  }
}

fn the_longest<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
  // life time b survive long than a
  if s1.len() > s2.len() {
    s1
  } else {
    s2
  }
}

fn struct_life_time() {
  // struct shorter than or equal member lifetime
  struct Foo<'a> {
    part: &'a str,
  }

  let word = String::from("Sometimes think, the greatest sorrow than older");
  let first = word.split(',').next().expect("Could not find a ','");
  let f = Foo { part: first };
  assert_eq!("Sometimes think", f.part);

  impl<'a> Foo<'a> {
    fn split_first(s: &'a str) -> &'a str {
      s.split(',').next().expect("Could not find a ','")
    }
    fn new(s: &'a str) -> Self {
      Foo {
        part: Foo::split_first(s),
      }
    }
    // self is a object
    // Self is type for self
    fn get_part(&self) -> &str {
      self.part
    }
  }

  let words = String::from("Somethimes think, the greatest sorrow than older");
  let foo = Foo::new(words.as_str());
  println!("{:?}", foo.get_part());
}

fn life_limit() {
  use std::fmt::Debug;
  #[derive(Debug)]
  struct Ref<'a, T: 'a>(&'a T);
  // T type ref life time == 'a

  fn print<T>(t: T)
  where
    T: Debug,
  {
    println!("'print': t is {:?}", t);
  }

  fn print_ref<'a, T>(t: &'a T)
  where
    T: Debug + 'a,
  {
    println!("'print_ref': t is {:?}", t);
  }

  let x = 7;
  let ref_x = Ref(&x);
  print_ref(&ref_x);
  print(ref_x);

  trait Foo {};
  struct Bar<'a> {
    _x: &'a i32,
  }
  impl<'a> Foo for Bar<'a> {}
  let num = 5;
  let box_bar = Box::new(Bar { _x: &num });
  let _obj = box_bar as Box<dyn Foo>;

  trait _Foo<'a> {}
  struct _FooImpl<'a> {
    s: &'a [u32],
  }
  impl<'a> _Foo<'a> for _FooImpl<'a> {}
  fn _foo<'a>(s: &'a [u32]) -> Box<dyn _Foo<'a> + 'a> {
    Box::new(_FooImpl { s: s })
  }
}

fn smart_pointer() {
  // copy
  let a = Box::new("hello");
  // move
  let b = Box::new("Rust".to_string());
  let c = *a;
  let d = *b;
  println!("a = {:?}", a);
  println!("c = {:?}", c);
  println!("d = {:?}", d);
  // println!("{:?}", b);

  use std::rc::Rc;
  use std::sync::Arc;
  let r = Rc::new("Rust".to_string());
  let a = Arc::new(vec![1.0, 2.0, 3.0]);
  // move occurs
  let x = &*r;
  println!("{:?}", x);
  let f = &*a;
  println!("{:?}", f);
}

fn rc_and_weak() {
  use std::rc::Rc;
  let x = Rc::new(5);
  let y1 = x.clone(); // add strong count
  let _y2 = y1.clone(); // add strong count
  println!("{:?}", Rc::strong_count(&x));
  let _w = Rc::downgrade(&x); // add weak count
  println!("{:?}", Rc::weak_count(&x));
  let _y3 = &*x; // no add count
  println!("{}", 100 - *x);
}

fn cell_and_ref_cell() {
  use std::cell::Cell;
  struct Foo {
    x: u32,
    y: Cell<u32>,
  }

  let foo = Foo {
    x: 1,
    y: Cell::new(3),
  };
  assert_eq!(1, foo.x);
  assert_eq!(3, foo.y.get());
  foo.y.set(5);
  assert_eq!(5, foo.y.get());

  use std::cell::RefCell;
  let x = RefCell::new(vec![1, 2, 3, 4]);
  println!("{:?}", x.borrow());
  x.borrow_mut().push(5);
  println!("{:?}", x.borrow());
  {
    let x = RefCell::new(vec![1, 2, 3, 4]);
    let mut _v = x.borrow_mut();
    _v.push(5);
    // panic
    // let _m = x.borrow_mut();
    println!("{:?}", _v);
  }
}

fn copy_on_write() {
  use std::borrow::Cow;
  fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
      // first call first create
      let v = input[i];
      if v < 0 {
        input.to_mut()[i] = -v;
      }
    }
  }

  fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    lst.iter().fold(0, |acc, &n| acc + n)
  }

  // no variable demand no clone
  let s1 = [1, 2, 3];
  let mut l1 = Cow::from(&s1[..]);
  abs_all(&mut l1);
  println!("IN {:?}", s1);
  println!("OUT {:?}", l1);

  // variable demand clone
  let s2 = [1, -2, 3];
  let mut l2 = Cow::from(&s2[..]);
  abs_all(&mut l2);
  println!("IN {:?}", s2);
  println!("OUT {:?}", l2);

  // no clone because v1 mutable
  let mut v1 = Cow::from(vec![1, -2, 3]);
  abs_all(&mut v1);
  println!("IN OUT {:?}", v1);

  // no variable demand no clone
  let s3 = [1, 3, 5, 6];
  let suml = abs_sum(&s3[..]);
  println!("{:?}", s3);
  println!("{:?}", suml);

  // variable demand clone
  let s4 = [1, -3, 5, -6];
  let sum2 = abs_sum(&s4[..]);
  println!("{:?}", s4);
  println!("{:?}", sum2);
}

fn token() {
  use std::borrow::Cow;
  use std::thread;
  #[derive(Debug)]
  struct Token<'a> {
    raw: Cow<'a, str>,
  }
  impl<'a> Token<'a> {
    pub fn new<S>(raw: S) -> Token<'a>
    where
      S: Into<Cow<'a, str>>,
    {
      Token { raw: raw.into() }
    }
  }

  let _token = Token::new("abs123");
  let token = Token::new("api.example.io".to_string());
  thread::spawn(move || {
    println!("token: {:?}", token);
  })
  .join()
  .unwrap();

  let raw = String::from("abc");
  // borrowed value does not live long enough
  // argument requires that `raw` is borrowed for `'static
  // let s = &raw[..];
  let s = raw;
  let token = Token::new(s);
  thread::spawn(move || {
    println!("token: {:?}", token);
  })
  .join()
  .unwrap();
}

fn none_lexical_lifetime() {
  println!("I don't understand o(╥﹏╥)o");
}
