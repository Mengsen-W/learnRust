mod closures;

fn main() {
    println!("Hello, world!");
    closures_test();
}

fn closures_test() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    closures::generate_workout(simulated_user_specified_value, simulated_random_number);
}
