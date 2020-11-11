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
}

fn test_const_and_static() {
    assert_eq!(MAX_NUM, MIN_NUM);
}
