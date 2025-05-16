#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected :String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let c:CipherError=CipherError{
        expected:original.to_string(),
    };
    if original.len()!=ciphered.len(){
        return Err(c);
    }

    let mut s:String=String::new();
    let new=original.to_lowercase();
    for ch in new.bytes(){
        if ch.is_ascii_alphabetic() {
            let pos = ch as i32 - 'a' as i32;
            let new_pos = 25 - pos;  
            let new_char = (new_pos + 'a' as i32) as u8 as char;
              s.push(new_char);
        }else{
            s.push(ch as char);
        }
    }  
    
    if s != ciphered.to_lowercase() {
        return Err(c);
    }

    Ok(())
}