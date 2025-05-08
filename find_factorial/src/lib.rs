pub fn factorial(num: u64) -> u64 {
    let mut x=1;
    for i in 1..num+1{
        x=x*i;
    }
    x
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
