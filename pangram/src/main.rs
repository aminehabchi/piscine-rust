use pangram::*;

fn main() {
    println!(
        "{}",
        is_pangram("own fox jumps over he lazy dog!")
    );
    println!("{}", is_pangram("Five quacking Zephyrs jolt my wax bed."));
}
