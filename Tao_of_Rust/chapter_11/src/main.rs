use std::cell::RefCell;
use std::panic;
use std::thread;
use std::thread::{current, Builder};

fn main() {
  {
    builder_thread_learning();
  }
  {
    thread_local_learning();
  }
}

fn builder_thread_learning() {
  let mut v = vec![];
  for id in 0..5 {
    let thread_name = format!("child-{}", id);
    let size: usize = 3 * 1024;
    let builder = Builder::new().name(thread_name).stack_size(size);
    let child = builder
      .spawn(move || {
        println!("in child: {}", id);
        #[allow(unused_must_use)]
        if id == 3 {
          panic::catch_unwind(|| {
            panic!("oh no");
          });
          println!("in {} do something", current().name().unwrap());
        }
      })
      .unwrap();
    v.push(child);
  }

  for child in v {
    child.join().unwrap();
  }
}

fn thread_local_learning() {
  thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
  FOO.with(|f| {
    assert_eq!(*f.borrow(), 1);
    *f.borrow_mut() = 2;
  });

  thread::spawn(|| {
    FOO.with(|f| {
      assert_eq!(*f.borrow(), 1);
      *f.borrow_mut() = 3;
    });
  });
  FOO.with(|f| {
    assert_eq!(*f.borrow(), 2);
  });
}
