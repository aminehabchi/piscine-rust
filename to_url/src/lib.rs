pub fn to_url(s: &str) -> String {
    let mut new_s:String=String::from("");
    for ch in s.chars(){
        if ch==' '{
            new_s.push_str("%20");
        }else{
            new_s.push(ch);
        }
    }
    new_s
}