pub fn is_pangram(s: &str) -> bool {
    let s =s.to_lowercase();
    
    for i in 0..26{
        let letter:char=(i+97) as u8 as char;
        if !s.contains(letter){
            return false;
        }
    }

    true
}