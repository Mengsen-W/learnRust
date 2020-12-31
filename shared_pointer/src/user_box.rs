pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn recursive_type() {
    use List::{Cons, Nil};
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
