use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // let v = vec![1, 2, 3];
    // v[99];
    // panic!("crash and burn");

    // result();
    // result2();
    // let e =  read_username_from_file();
    // let e = match e {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem creating thre file: {:?}", error),
    // };

    let _f = File::open("hello.txt")?;

    Ok(())
}

#[allow(dead_code)]
fn result() {
    let _f = File::open("hello.txt");

    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

#[allow(dead_code)]
fn result2() {
    // let f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    let _f = File::open("hello.txt");

    let mut _f = match _f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut _s = String::new();

    // match _f.read_to_string(&mut _s) {
    //     Ok(_) => Ok(_s),
    //     Err(e) => Err(e),

    // };
    _f.read_to_string(&mut _s)?;
    Ok(_s)
}

#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

#[allow(dead_code)]
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
