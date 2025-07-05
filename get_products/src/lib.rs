pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return vec![];
    }
    let all: usize = arr.iter().product();
    let mut ar = arr.clone();
    for i in 0..ar.len() {
        ar[i] = all / ar[i];
    }
    ar
}
