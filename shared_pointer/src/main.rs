mod user_box;
mod user_deref;
mod user_drop;
mod user_ref;
mod user_refcall;
mod reference_cycles;

fn main() {
    println!("Hello, world!");
    user_box::recursive_type();
    user_deref::my_box();
    user_drop::my_drop();
    user_ref::my_ref();
    user_refcall::my_refcall();
    reference_cycles::cycles();
}
