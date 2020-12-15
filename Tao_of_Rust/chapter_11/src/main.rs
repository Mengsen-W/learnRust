use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock};
use std::thread;
use std::thread::{current, Builder};
use std::time::Duration;

fn main() {
  {
    thread_manger();
  }
}

fn thread_manger() {
  {
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
  {
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
  {
    let parked_thread = thread::Builder::new()
      .spawn(|| {
        println!("Parking thread");
        thread::park();
        println!("Thread unparked");
      })
      .unwrap();
    thread::sleep(Duration::from_millis(10));
    println!("Unparked the thread");
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
  }
  {
    let s = Arc::new(Mutex::new("Hello".to_string()));
    let mut v = vec![];
    for _ in 0..3 {
      let s_clone = s.clone();
      let child = thread::spawn(move || {
        let mut s_clone = s_clone.lock().unwrap();
        s_clone.push_str(" Rust");
      });
      v.push(child);
    }
    for child in v {
      child.join().unwrap();
    }
  }
  {
    let mutex = Arc::new(Mutex::new(1));
    let c_mutex = mutex.clone();
    let _ = thread::spawn(move || {
      let mut data = c_mutex.lock().unwrap();
      *data = 2;
      panic!("oh no");
    })
    .join();

    assert_eq!(mutex.is_poisoned(), true);
    match mutex.lock() {
      Ok(_) => unreachable!(),
      Err(p_err) => {
        let data = p_err.get_ref();
        println!("recovered: {}", data);
      }
    };
  }
  {
    let lock = RwLock::new(5);
    {
      let r1 = lock.read().unwrap();
      let r2 = lock.read().unwrap();
      assert_eq!(*r1, 5);
      assert_eq!(*r2, 5);
    }
    {
      let mut w = lock.write().unwrap();
      *w += 1;
      assert_eq!(*w, 6);
    }
  }
  {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    for _ in 0..5 {
      let c = barrier.clone();
      handles.push(thread::spawn(move || {
        println!("before wait");
        c.wait();
        println!("after wait");
      }));
    }
    for handle in handles {
      handle.join().unwrap();
    }
  }
  {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = pair.clone();
    thread::spawn(move || {
      let &(ref lock, ref cvar) = &*pair_clone;
      let mut started = lock.lock().unwrap();
      *started = true;
      cvar.notify_one();
    });
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
      println!("{}", started); // false
      started = cvar.wait(started).unwrap();
      println!("{}", started); // true
    }
  }
}
