pub fn capitalize_first(input: &str) -> String {
    if input.len()==0{
        return "".to_string();
    }
    let mut bytes: Vec<u8> = input.bytes().collect();
    if bytes[0]>=97 && bytes[0]<=122{
        bytes[0]-=32;
    }  
                 //return  Option
    let s = String::from_utf8(bytes);
     s.unwrap_or("".to_string())
}

pub fn title_case(input: &str) -> String {
    if input.len()==0{
        return "".to_string();
    }
    let mut bytes: Vec<u8> = input.bytes().collect();
    if bytes[0]>=97 && bytes[0]<=122{
        bytes[0]-=32;
    } 

    for i in 1..bytes.len(){
        if i>0{
            if bytes[i-1]==32{
                if bytes[i]>=97 && bytes[i]<=122{
                    bytes[i]-=32;
                }
            }
        }

    }
                 //return  Option
    let s = String::from_utf8(bytes);
     s.unwrap_or("".to_string())
}

pub fn change_case(input: &str) -> String {
    if input.len()==0{
        return "".to_string();
    }
    let mut bytes: Vec<u8> = input.bytes().collect();
  
    for i in 0..bytes.len(){
        if bytes[i]>=97 && bytes[i]<=122{
            bytes[i]-=32;
         }else if bytes[i]>=65 && bytes[i]<=90{
            bytes[i]+=32;
        }

    }
                 //return  Option
    let s = String::from_utf8(bytes);
     s.unwrap_or("".to_string())
}