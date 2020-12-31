pub fn incr(x: u32) -> u32 {
    x + 1
}

#[macro_export]
macro_rules! inc {
    ($x: expr) => {
        hashmap_lite::incr_marco::incr($x)
    };
}
