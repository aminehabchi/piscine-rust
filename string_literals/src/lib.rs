pub fn is_empty(v: &str) -> bool {
    v.len()==0
}

pub fn is_ascii(v: &str) -> bool {
    for ch in v.bytes(){
        if ch>255{
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    let l1=v.len();
    let l2=pat.len();
    let mut chars1=v.chars();
    let mut chars2=pat.chars();
    for i in 0..l1-l2+1{
        let mut bl=true;
        for j in 0..l2{
            if chars1.nth(i+j)!=chars2.nth(j){
                bl=false;
                break;
            }
        }

        if bl{
            return true;
        }
    }

    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
   v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    for (i,ch) in v.chars().enumerate(){
        if ch==pat{
            return i;
        }
    }
    0
}