pub fn first_fifty_even_square() -> Vec<i32> {
    let mut arr: Vec<i32> = vec![];
    let mut i: i32 = 2;
    while arr.len() != 50 {
        arr.push(i * i);
        i += 2;
    }
    arr
}
