// Any value is either A or B, and it can't be both

enum IpAddrKind {
    // V4(u8, u8, u8, u8),
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32  },
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

fn main() {
    let a;
    a = 0;
    // a = 1;
    println!("{}",a);
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

fn route(ip_type: IpAddrKind) { }
