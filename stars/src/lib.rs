pub fn stars(n: u32) -> String {
    let mut s:String=String::new();
    for i in 0..=n*n{
        s.push('*');
    }
    return s;
}