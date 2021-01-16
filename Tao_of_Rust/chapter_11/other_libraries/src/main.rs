extern crate rayon;
use rayon::prelude::*;

extern crate crossbeam;
use crossbeam::channel;
use crossbeam::thread::scope;

fn rayon_learning() {
    {
        fn sum_of_squares(input: &[i32]) -> i32 {
            input.par_iter().map(|&i| i * i).sum()
        }
        fn increment_all(input: &mut [i32]) {
            input.par_iter_mut().for_each(|p| *p += 1);
        }
        let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let r = sum_of_squares(&v);
        println!("{}", r);
        let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        increment_all(&mut v);
        println!("{:?}", v);
    }
    {
        fn fib(n: i32) -> u32 {
            if n < 2 {
                return n as u32;
            }
            let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));
            a + b
        }
        let r = fib(32);
        assert_eq!(r, 2178309);
    }
}

fn leaning_crossbeam() {
    {
        let array = [1, 2, 3];
        scope(|scope| {
            for i in &array {
                scope.spawn(move |_| {
                    println!("element: {}", i);
                });
            }
        })
        .unwrap();
    }
    #[allow(unused_must_use)]
    {
        let (s, r) = channel::unbounded();
        crossbeam::scope(|scope| {
            scope.spawn(|_| {
                s.send(1).unwrap();
                r.recv().unwrap();
            });
            scope.spawn(|_| {
                s.send(2).unwrap();
                r.recv().unwrap();
            });
        });
    }
    // {
    //     fn fibonacci(
    //         fib: channel::Sender<u64>, quit: channel::Receiver<()>
    //     ) {
    //         let (mut x, mut y) = (0, 1);
    //         loop {
    //             select!{
    //                 send(fib, x) => {
    //                     let tmp = x;
    //                     x = y;
    //                     y = tmp + y;
    //                 }
    //                 recv(quit) => {
    //                     println!("quit");
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    //     let (fib_s, fib_r) = channel::unbounded(0);
    //     let (quit_s, quit_r) = channel::unbounded(0);
    //     thread::spawn(move || {
    //         for _ in 0..10 {
    //             println!("{}", fib_r.recv().unwrap());
    //         }
    //         quit_s.send(());
    //     });
    //     fibonacci(fib_s, quit_r);
    // }
}

fn main() {
    {
        rayon_learning();
    }
    {
        leaning_crossbeam();
    }
    println!("Hello, world!");
}
