// Any value is either A or B, and it can't be both

enum IpAddrKind {
    // V4(u8, u8, u8, u8),
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    //
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => {
            println!("Lucky Nickel");
            5
        }
        Coin::Dime => {
            println!("Lucky Dime");
            10
        }
        Coin::Quarter(state) => {
            println!("Lucky Quarter {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    if_let();

    let a;
    a = 0;
    // a = 1;
    println!("{}", a);
    println!("Hello, world!");
    define_enum();

    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    // use Option value must convert to determine value
    // no implementation for `i8 + std::option::Option<i8>`
    // let sum = x + y;
}

fn define_enum() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let _home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));
    route(_home);

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_type: IpAddrKind) {}

fn if_let() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
