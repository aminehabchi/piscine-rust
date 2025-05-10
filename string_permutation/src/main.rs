use string_permutation::*;

fn main() {
    let word = "thought";
    let word1 = "thgth";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}