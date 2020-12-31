fn main() {
    loop {
        println!("Please input function type:");
        println!("1 = convert temperatures");
        println!("2 = Generate the nth Fibonacci number");
        println!("3 = Print the lyrics to the Christmas carol");
        println!("0 = exit");

        let mut enum_ = String::new();

        std::io::stdin()
            .read_line(&mut enum_)
            .expect("Fail to read line");
        let enum_: u32 = match enum_.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Fail convert typea... continue");
                continue;
            }
        };

        if enum_ == 0 {
            println!("see you again");
            return;
        } else if enum_ == 1 {
            convert_temperatures();
        } else if enum_ == 2 {
            fibonacci_sequence();
        } else if enum_ == 3 {
            let c: bool = christmas_song(5);
            if c == true {
                println!("Yes, you gotted");
            } else {
                println!("something wrong maybe, come on!");
            }
        } else {
            println!("input from 0 to 3");
            continue;
        }
    }
}

fn convert_temperatures() {
    println!("Please input type");
    println!("1 = C, 2 = F");

    let mut type_ = String::new();
    std::io::stdin()
        .read_line(&mut type_)
        .expect("Fail to read line");
    let type_: u32 = match type_.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Fail convert typea... continue");
            return;
        }
    };

    if type_ == 1 {
        println!("Please input you temperatures");
        let mut type_ = String::new();
        std::io::stdin()
            .read_line(&mut type_)
            .expect("Fail to read line");
        let type_: f64 = match type_.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Fail convert typea... continue");
                return;
            }
        };
        println!("The result = {}", (type_ * 1.8) + 32.0);
    } else if type_ == 2 {
        println!("Please input you temperatures");
        let mut type_ = String::new();
        std::io::stdin()
            .read_line(&mut type_)
            .expect("Fail to read line");
        let type_: f64 = match type_.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Fail convert typea... continue");
                return;
            }
        };
        println!("The result = {}", (type_ - 32.0) / 1.8);
    }
}

fn fibonacci_sequence() {
    println!("input number of fibonacci sequence");
    let mut type_ = String::new();
    std::io::stdin()
        .read_line(&mut type_)
        .expect("Fail to read line");
    let type_: u32 = match type_.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Fail convert typea... continue");
            return;
        }
    };

    let mut fab_one = 0;
    let mut fab_two = 1;
    let mut fab_last = 1;

    if type_ == 0 {
        println!("the value = {}", 0);
    } else if type_ == 1 {
        println!("the value = {}", 1);
    } else {
        for _i in 0..type_ {
            fab_last = fab_one + fab_two;
            fab_one = fab_two;
            fab_two = fab_last;
        }
        println!("the value = {}", fab_last);
    }
}

fn christmas_song(_x: u32) -> bool {
    for _i in 0.._x {
        println!("The value = 1{}", _i);
    }
    return true;
}
