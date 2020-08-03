struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct no filed name
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct unit;

fn main() {
    // We don’t have to specify the fields in the same order 
    // in which we declared them in the struct.
    
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("somebody"),
        active: true,
        sign_in_count: 1,
    }; 

    // Note that the entire instance must be mutable
    // Rust doesn’t allow us to mark only certain fields as mutable
    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("anotherUserName");

    let _user_fn = build_user(String::from("user_fn@example.com"), String::from("user_fn"));

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,

    };
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // another same the user1
        ..user1

    };

    // tuple
    let _x: (i32, f64, u8) = (500, 6.4, 1);
    // tuple struct
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    println!("Hello, world!");
}

fn build_user(email: String, username: String) -> User {
    //Because the email field and the email parameter have the same name,
    //we only need to write email rather than email: email.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,

    }
}
