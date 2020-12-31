fn main() {
    println!("Hello, world!");
    if_flow();
    else_if_flow();
    let_if();
    loop_flow();
    while_flow();
    for_flow();
}

fn if_flow() {
    let num: i32 = 3;

    // must be bool
    // if num {
    //     println!("error with expected bool, found integer");
    // }
    if num < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    if num != 0 {
        println!("number was something other than zero");
    }
}

fn else_if_flow() {
    let number = 6;

    // if true flow don't going on this
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_if() {
    let condition: bool = true;
    //  values that have the potential to be results from each arm of the if must be the same type
    let number = if condition { 5 } else { 6 };
    // expected integer, found `&str`
    // let number = if condition { 5 } else { "six" }
    println!("The value of number is: {}", number);
}

fn loop_flow() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            // break will return with statement
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_flow() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn for_flow() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // [1, 4)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
