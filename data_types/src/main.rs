fn main() {
    let _guess: u32 = "42".parse().expect("Not a number");

    // usize and isize epend on the kind of computer your program is running
    // 64 bits if you’re on a 64-bit architecture
    let arch: usize = 1000_000;
    println!("usize arch = {}", arch);

    // integer types default to i32

    let decimal: u32 = 98_222;
    println!("u32 decimal = {}", decimal);
    let hex: i32 = 0xff;
    println!("i32 hex = {}", hex);
    let octal: i32 = 0o77;
    println!("i32 octal= {}", octal);
    let binary: u8 = 0b1111_0000;
    println!("u8 binary = {}", binary);
    let byte: u8 = b'a';
    println!("u8 byte = {}", byte);

    // value outside of that range
    // in debug mode programe compile with error "panic"
    // in release flag compile performs two’s complement wrapping


    // default float type is f64
    let x = 2.0; //f64
    println!("f64 x = {}", x);
    let y: f32 = 1.0; //f32
    println!("f32 y = {}", y);

    // addition
    // no implementation for `{integer} + {float}`
    let sum = 5 + 10;
    println!("sum = {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("product = {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient = {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("remainder = {}", remainder);

    // expected `bool`, found integer
    // just "true" and "false"
    let f: bool = false; 
    println!("bool f = {}", f);

    // char literals are specified with single quotes
    // as opposed to string literals, which use double quotes
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    // which means it can represent a lot more than just ASCII.
    let c: char = 'C';
    println!("char c = {}", c);

    // Tuples have a fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("tup1 = {}, tup2 = {}, tup3 = {}", _x, _y, _z);
    //destructuring through pattern matching, directly by using a period (.)
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("tup1 = {}, tup2 = {}, tup3 = {}", five_hundred, six_point_four, one);

    // every element of an array must have the same type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5]; mean let a = [3, 3, 3, 3, 3]
    let first = a[0];
    println!("a[0] = {}", first);
    let second = a[1];
    println!("a[1] = {}", second);
    
    // if you try to access an element of an array
    // index out of bounds: the len is 5 but the index is 10
    
    // When you attempt to access an element using indexing, Rust will check that the index you’ve
    // specified is less than the array length. If the index is greater than or equal to the array
    // length, Rust will panic.

    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
}
