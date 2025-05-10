use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut m1: Vec<u8>=s1.bytes().collect();
    let mut m2: Vec<u8>=s2.bytes().collect();
    m1.sort();
    m2.sort();

    m1==m2
}