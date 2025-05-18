pub fn scytale_cipher(message: String, i: u32) -> String {
    if message==""{
        return "".to_string();
    }
    let mut s:String=String::new();
    let m:Vec<u8>=message.bytes().collect();
    let l:usize=m.len();

    for index in 0..i as usize{
        s.push(m[index] as char);
        if index+(i as usize)<l{
             s.push(m[index+i as usize] as char);
        }else{
            s.push(' ');
        }
    } 

    let s= s.trim_end_matches(' ').to_string();

    s
}