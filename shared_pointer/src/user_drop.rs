struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);

    }
}

pub fn my_drop() {
    let _c = CustomSmartPointer { data: String::from("my stuff")  };
    drop(_c);
    println!("CustomSmartPointer dropped before the end of main.");
    let _d = CustomSmartPointer { data: String::from("other stuff")  };
    println!("CustomSmartPointers created.");
}
