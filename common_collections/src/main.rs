mod vec;
mod string;
mod hash;

fn main() {
    println!("Hello, world!");
    vec_fn();
    string_fn();
    hash_fn();
}

fn vec_fn() {
    vec::vec_define();
    vec::read_vec();
    vec::traverse_vec();
    vec::diff_type_vec();
}

fn string_fn() {
    string::string_define();
    string::string_update();
    string::string_index();
    string::string_traverse();
    string::print_hell();
}

fn hash_fn() {
    hash::hash_define();
    hash::hash_get();
    hash::hash_traverse();
    hash::hash_update();
}
