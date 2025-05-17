fn is_cap(t: &str) -> bool {
    let has_letters = t.chars().any(|c| c.is_alphabetic());
    has_letters && t.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}

pub fn talking(text: &str) -> &str {
    if text.trim().is_empty(){
     return "Just say something!";
    }

    if is_cap(text) && text.contains('?'){
        return "Quiet, I am thinking!";
    }

    if text.contains('?'){
        return "Sure.";
    }

   if is_cap(text){
    return "There is no need to yell, calm down!";
   }


   "Interesting"
}