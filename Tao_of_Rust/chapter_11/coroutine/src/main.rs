#![feature(generators, generator_trait)]
use std::ops::Generator;

fn main() {
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
