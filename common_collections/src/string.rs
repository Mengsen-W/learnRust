pub fn string_define() {
    let mut _s = String::new();
    let _s = String::from("initial contents");
    let data = "initial contents";

    let _s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let _s = "initial contents".to_string();
} 

pub fn string_update() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str get ownership of s2
    s1.push_str(s2);
    // can not use s2
    println!("s2 is {}", s2);
    println!("{}", s1);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

}

pub fn string_index() {
    let _s1 = String::from("hello");
    // return char index, but some lang 2 char with a character
    // this will error in h, so rust ban this operator
    // let h = s1[0];
    let hello = "Здравствуйте";
    println!("{}", hello);
    // let s = &hello[0..4];
}

pub fn string_traverse() {
    for c in "नमस्ते".chars() {
        println!("{}", c);

    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);

    }
}

pub fn print_hell () {
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);
}
