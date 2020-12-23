extern crate rayon;
use rayon::prelude::*;

extern crate crossbeam;
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
            let (a, b) = rayon::join(
                || fib(n - 1), || fib(n - 2)
            );
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
        }).unwrap();
    }
    {}
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
