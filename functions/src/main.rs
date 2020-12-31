fn main() {
    println!("Hello, world!");
    another_function(500, 1_000);

    // Statements are instructions that perform some action and do not return a value
    // Expressions evaluate to a resulting value
    // statements
    let x = 3;
    println!("x = {}", x);

    let y = {
        // expression
        // Expressions do not include ending semicolons
        // If you add a semicolon to the end of an expression, you turn it into a statement
        // Expressions can be part of statements
        // Calling a function is an expression
        // Calling a macro is an expression
        // {} is an expression
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("x = {}", x);

    let x = plus_one(x);
    println!("x = {}", x);

    println!("new value = {}", plus_one(x));

    // if statement writed in the end of function, error with expected `i32`, found `()`
}

// Rust doesn’t care where you define your functions
fn another_function(x: i32, y: i32) {
    // In function signatures, you must declare the type of each parameter
    println!(
        "another_function first paramter {} and seconde parameter {}",
        x, y
    );
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
