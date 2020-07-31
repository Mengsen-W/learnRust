const MAX_POINTS: u32 = 1_000_000;
fn main() {
    let mut x = 5;
    println!("The value x = {}", x);

    // cannot assign twice to immutable variable
    x = 6;
    println!("The value x = {}", x);

    // const variable similar to the "static const" in c++
    println!("The value const and declared in any scope = {}", MAX_POINTS);

    // we’re effectively creating a new variable when we use the let keyword again
    let shadow = 5;
    let shadow = shadow + 1;
    println!("The value shadow = {}", shadow);

    // change the type of the value
    let str = "abcdefg";
    println!("The value str = {}", str);
    let str = str.len();
    println!("The change type value str = {}", str);

    // expected `&str`, found `usize`
    // let mut str = "abdc";
    // str = str.len();

}
