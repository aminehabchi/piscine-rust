pub fn fibonacci(n: u32) -> u32 {
    if n==0{
        return 0;
    }
    if n==1{
        return 1;
    }
    if n==2{
        return 1;
    }
    return fibonacci(n-1)+fibonacci(n-2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
