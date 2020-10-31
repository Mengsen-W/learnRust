/// @Brief:proof of work
/// @Author: mengsen
/// @Last Modified: 2020-10-31
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn main() {
    let mut hasher = DefaultHasher::new();
    let mut data: u64 = 0;
    // proof of work will adjust this proof dynamic
    // and can make it calculate for 10 minutes
    let mut proof = String::from("1");

    loop {
        loop {
            data.hash(&mut hasher);
            if hasher.finish().to_string()[0..proof.len()] != proof {
                data = data + 1;
            } else {
                println!("The data = {}", data);
                println!("The hasher = {}", hasher.finish().to_string());
                proof.push('1');
                break;
            }
        }
        if data >= 1000_000 {
            println!("To big number for calculating... exit");
            break;
        }
    }
}
