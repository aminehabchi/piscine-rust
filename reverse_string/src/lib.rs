pub fn rev_str(input: &str) -> String {
    let mut i:usize=0;
    let mut s = String::from("");
    let chars: Vec<char> = input.chars().collect(); 
    let l=chars.len();
  
    while i<l{
        let index:usize=l-1-i;
        s.push(chars[index]);
        i+=1;
    }
   
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
