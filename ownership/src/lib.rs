pub fn first_subword(mut s: String) -> String {
     for (i,ch) in s.chars().enumerate(){
        if i!=0 && ((ch>='A' && ch<='Z') || ch=='_'){
            return s[..i].to_string()
        }
    }
    s[..].to_string()
}