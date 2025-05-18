pub fn scytale_cipher(message: String, i: u32) -> String {
        if i<=1 ||  message==""{
        return message;
    }

    let mut s:String=String::new();
    let m:Vec<u8>=message.bytes().collect();
    let l:usize=m.len();   

    let cols:usize= (l as f64 / i as f64).ceil() as usize;
    let row: usize=i as usize;
    
    for index in 0..row as usize{
        for col in 0..cols{
            if index+row*col<l{
                    s.push(m[index+row*col] as char);
                }else{
                    s.push(' ');
                }
            }
        } 


 let s= s.trim_end_matches(' ').to_string();

    s
}