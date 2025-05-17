pub fn stars(n: u32) -> String {
    let mut s:String=String::new();
    let nn:u32=2_u32.pow(n);
    for i in 0..nn{
        s.push('*');
    }
    return s;
}