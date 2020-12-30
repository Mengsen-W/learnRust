pub mod incr_marco;

#[macro_export]
macro_rules! hashmap {
  (@unit $($x:tt)*) => (());
  (@count $($rest:expr), *) =>
    (<[()]>::len(&[$(hashmap!(@unit! $rest)),*]));
  ($($key: expr => $value: expr), * $(,)*) => {
    {
      let _cap = hashmap!(@count $($key),*);
      let mut _map = ::std::collections::HashMap::with_capacity(_cap);
    $(_map.insert($key, $value);)*
    _map
    };
  }
}