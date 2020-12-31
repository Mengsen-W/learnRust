#![allow(unused)]
mod define_shared_behavior;
mod generic_data_types;
use define_shared_behavior::Summary;

fn main() {
    println!("Hello, world!");
}

fn largest_test() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = generic_data_types::largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = generic_data_types::largest(&char_list);
    println!("The largest char is {}", result);
}

fn data_types() {
    let p = generic_data_types::Point { x: 5, y: 4.0 };
    println!("p.x = {}", p.x());

    let pf = generic_data_types::Point { x: 5.0, y: 4.0 };
    println!("distance_from_origin {}", pf.distance_from_origin());

    let p1 = generic_data_types::Point { x: 5, y: 10.4 };
    let p2 = generic_data_types::Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn summarize_test() {
    let tweet = define_shared_behavior::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("backup function {}", tweet.summarize_other());
    println!("default {}", tweet.summarize_default());

    let vec = vec![1, 2];
    println!("{}", vec.summarize());
}
