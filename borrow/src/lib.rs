// s.chars().count()

pub fn str_len(s: &str) -> usize {
    let mut chars=s.chars();
    let mut counter:usize=0;

    loop{
        let c=chars.next();
        if c==None{
            break;
        }
        counter+=1;
   } 

   counter
}