pub fn search(array: &[i32], key: i32) -> Option<usize> {
    // array.iter().position(|&x| x == key);
    for i in 0..array.len(){
        if key==array[i]{
            return Some(i);
        }
    }
    None
}