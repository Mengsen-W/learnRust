use hashmap_lite::{hashmap, inc};

#[macro_use]
mod r#macro {
  macro_rules! X { () => { Y!(); } }
  macro_rules! Y { () => {} }
}

fn main() {
  let map = hashmap!{
    "a" => 1,
    "b" => 2,
  };
  assert_eq!(map["a"], 1);
  X!();
  assert_eq!(2, inc!(1));
}
