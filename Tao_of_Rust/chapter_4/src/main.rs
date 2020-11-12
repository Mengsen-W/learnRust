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
}

fn test_const_and_static() {
    assert_eq!(MAX_NUM, MIN_NUM);
}

const fn return_num(num: i32) -> i32 {
    return num;
}
