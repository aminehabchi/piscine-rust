pub fn sum(a:&[i32; 10]) -> i32 {
    let mut sum :i32=0;
    for val in a{
        sum+=val;
    }
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let a:[i32; 32]=[10; 32];
    a
}
