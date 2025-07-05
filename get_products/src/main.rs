use get_products::get_products;

fn main() {
    let arr: Vec<usize> = vec![1];
    let output = get_products(arr);
    println!("{:?}", output);
}