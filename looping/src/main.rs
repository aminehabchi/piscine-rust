use std::io;

fn main() {
    let mut i = 0;
    let mut input = String::new();
    let answer: String = "The letter e\n".to_string();
    loop {
        i+=1;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin().read_line(&mut input).unwrap();
        if input==answer{
            break;
        }
        input="".to_string();
    }
    println!("Number of trials: {}",i)
}
