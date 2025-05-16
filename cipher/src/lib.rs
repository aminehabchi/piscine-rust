#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected :String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
   
    let mut s:String=String::new();
   
    for ch in original.bytes(){
        if ch.is_ascii_alphabetic() {
            let mut base='a';
            if ch!=ch.to_ascii_lowercase(){
                base='A';
            }
            let pos = ch as i32 - base as i32;
            let new_pos = 25 - pos;  
            let new_char = (new_pos + base as i32) as u8 as char;
              s.push(new_char);
        }else{
            s.push(ch as char);
        }
    }  
    let c:CipherError=CipherError{
        expected:s.to_string(),
    };

    if s != ciphered {
        return Err(c);
    }

    Ok(())
}