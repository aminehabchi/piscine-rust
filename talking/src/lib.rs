fn is_cap(t :&str)->bool{
    let s=t.to_uppercase();
    return t==s;
}

pub fn talking(text: &str) -> &str {
    if text==""{
     return "Just say something!";
    }
    if is_cap(text) && text.contains('?'){
        return "Quiet, I am thinking!";
    }

    if !is_cap(text) && text.contains('?'){
        return "Sure.";
    }

   if is_cap(text){
    return "There is no need to yell, calm down!";
   }


   "Interesting"
}