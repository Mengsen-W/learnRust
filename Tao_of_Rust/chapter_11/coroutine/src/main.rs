#![feature(generators, generator_trait)]
use std::ops::Generator;

fn main() {
    {
    let mut gen = || {
        yield 1;
        yield 2;
        yield 3;
        return 4;
    };

    unsafe {
        for _ in 0..4 {
            let c = gen.resume();
            println!("{:?}", c);
        }
    }
    println!("Hello, world!");
    }

    {
        pub fn up_to() -> impl Generator<Yield = u64, Return = ()> {
            || {
                let mut x = 0;
                loop {
                    x += 1;
                    yield x;
                }
                return ();
            }
        }

        let mut gen =up_to();
        unsafe{
            for _ in 0..10 {
                match gen.resume() {
                    GeneratorState::Yielded(i) => println!("{:?}", i),
                    _ => println!("Completed"),
                }
            }
        }

        {
            fn up_tp(limit: u64) -> impl Generator<Yield = (), Return = Result<u64, ()>> {
                move || for x in 0..limit {
                    yield();
                };
                return Ok(limit);
            }
            let limit = 2;
            let mut gen = up_to(limit);
            unsafe {
                for i in 0..=limit {
                    match gen.resume() {
                        GeneratorState::Yielded(v) => println!("resume {:?}: Pending", i),
                        GeneratorState::Completed(v) => println!("resume {:?}: Ready", i),
                    }
                }
            }
        }
    }
    {
        use std::pin::{Pin, Unpin};
        use std::marker::Pinned;
        use std::ptr::NonNull;
        struct Unmovable {
            data: String,
            slice: NonNull<String>,
            _pin: Pinned,
        }

        impl Unpin for Unmovable {}
        impl Unmovable {
            fn new(data: String) -> Pin<Box<Self>> {
                let res = Unmovable {
                    data,
                    slice: NonNull::dangling(),
                    _pin: Pinned,
                };
                let mut boxed = Box::pinned(res);
                let slice = NonNull::from(&boxed.data);
                unsafe {
                    let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                    Pin::get_mut_unchecked(mut_ref).slice = slice;
                }
                boxed
            }
        }

        let unmoved = Unmovable::new("Hello".to_string());
        let mut still_unmoved = unmoved;
        assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
        let mut new_unmoved = Unmovable::new("World".to_string());
        std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
    }

    {
        #![feature(arbitrary_self_types, futures_api)]
        #![feature(async_await, await_macro, pin)]
        use futures::{
            executor::ThreadPool,
            task::SpawnExt
        };
        use std::future::{Future};
        use std::pin::Pin;
        use std::task::*;
        pub struct AlmostReady {
            ready: bool,
            value: i32,
        }
        impl Future for AlmostReady {
            type Output = i32;
            fn poll(self: Pin<&mut Self>, lw: &LocalWaker) -> Poll<Self::Output> {
                if self.ready {
                    Poll::Ready(self.value + 1)
                } else {
                    unsafe { Pin::get_mut_unchecked(self).ready = true; }
                    lw.wake();
                    Poll::Pending
                }
            }
        }

        let mut executor = ThreadPool::new().unwrap();
        let future = async {
            println!("howdy!");
            let x = await!(almost_ready(5));
            println!("done: {:?}", x);
        };
        executor.run(future);
    }
}
