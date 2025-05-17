fn is_cap(t :&str)->bool{
    let s=t.to_uppercase();
    return t==s;
}

pub fn talking(text: &str) -> &str {
    if text.trim().is_empty(){
     return "Just say something!";
    }

    if is_cap(text) && text.ends_with('?'.to_string().as_str()){
        return "Quiet, I am thinking!";
    }

    if  text.ends_with('?'.to_string().as_str()) {
        return "Sure.";
    }

   if is_cap(text){
    return "There is no need to yell, calm down!";
   }


   "Interesting"
}