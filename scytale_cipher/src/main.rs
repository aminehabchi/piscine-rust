use scytale_cipher::scytale_cipher;

fn main() {
    println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher(String::from("abcdefg"), 2));
    // println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher(String::from("scytale Code"), 8));
}