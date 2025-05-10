use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut x:i32=-1;
    for (_,value) in h{
        if value>x{
            x=value;
        }
    }
    x
}