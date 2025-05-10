use arrays::*;

fn main() {
    // vec
    let mut binding = (1..=10).collect::<Vec<_>>();
    
    let a = &binding;
    let b =thirtytwo_tens();
    println!("{:?}",binding);
    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}