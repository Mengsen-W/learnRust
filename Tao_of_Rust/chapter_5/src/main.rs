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
    life_limit();
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
