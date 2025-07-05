use iterators::*;

fn main() {
    let c = Collatz::new(133);
    for (i, val) in c.enumerate() {
        println!("Step {}: {}", i, val.v);
    }
    println!("{:?}", collatz(0));
    println!("{:?}", collatz(1));
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(133));
}
