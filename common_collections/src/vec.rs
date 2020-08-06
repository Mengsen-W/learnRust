pub fn vec_define() {
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];
    let mut v3: Vec<i32> = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(2);
    {
        let _v = vec![1, 2, 3, 4];
    }
}

pub fn read_vec() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get return Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // panic wil exit
    // let _does_not_exist = &v[100];
    let _does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    // may be reference error
    let first = &v[0];
    println!("The first element is {}", first);
    // so complier can allow this
    // mutable borrow occurs here
    v.push(6);
}

pub fn traverse_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);

    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
            *i += 50;
            
    }
}

pub fn diff_type_vec() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),

    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),

    ];

    for i in &row {
        println!("{:?}", i);

    }
}
