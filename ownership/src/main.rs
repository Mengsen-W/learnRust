fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);

    // just one mut reference or mutilply non-mut reference
    // non-mut reference cannot Involved in the mut reference scope
    // cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{} and {} and {}",r1, r2, r3);
    println!("Hello, world!");
    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // string literals is silce non-mut reference
    let word = first_word(my_string_literal);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
    // return &s[..];
}
