pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let all: usize = arr.iter().product();
    let mut ar = arr.clone();
    for i in 0..ar.len() {
        ar[i] = all / ar[i];
    }
    ar
}
