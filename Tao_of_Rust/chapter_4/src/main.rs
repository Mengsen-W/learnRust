/// chapter 4 memory manger
/// mengsen
/// 2020-11-11 17:49:13

// inline and all runtime life
const MAX_NUM: u32 = 1;

// not inline and all runtime life
static MIN_NUM: u32 = 1;

fn main() {
    {
        test_const_and_static();
    }

    {
        // let a = 2;
        // let array: [i32; a] = [5; return_num(2) as usize];
        // non-constant value
        let array: [i32; return_num(2) as usize] = [5; return_num(2) as usize];
        println!("{:?}", array)
    }

    {
        mem_leak();
    }
    {
        mem_distribution();
    }
}

fn test_const_and_static() {
    assert_eq!(MAX_NUM, MIN_NUM);
}

const fn return_num(num: i32) -> i32 {
    return num;
}

fn mem_leak() {
    use std::cell::RefCell;
    use std::rc::Rc;
    type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;
    #[allow(dead_code)]
    struct Node<T> {
        data: T,
        next: NodePtr<T>,
    }
    impl<T> Drop for Node<T> {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }

    let first = Rc::new(RefCell::new(Node {
        data: 1,
        next: None,
    }));
    let second = Rc::new(RefCell::new(Node {
        data: 2,
        next: Some(first.clone()),
    }));
    // circular reference
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(first.clone());

    // no dropping call
}

#[allow(dead_code)]
fn mem_distribution() {
    struct A {
        a: u32,
        b: Box<u64>,
    }
    struct B(i32, f64, char);
    struct N;
    enum E {
        H(u32),
        M(Box<u32>),
    }
    union U {
        u: u32,
        v: u64,
    }

    println!("Box<u32>: {:?}", std::mem::size_of::<Box<u32>>());
    println!("A: {:?}", std::mem::size_of::<A>());
    println!("B: {:?}", std::mem::size_of::<B>());
    println!("N: {:?}", std::mem::size_of::<N>());
    println!("E: {:?}", std::mem::size_of::<E>());
    println!("U: {:?}", std::mem::size_of::<U>());
}
