pub fn is_empty(v: &str) -> bool {
    v.len()==0
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    let l1 = v.len();
    let l2 = pat.len();
    if is_empty(pat) {
        return true;
    }
    if l1 < l2 {
        return false;
    }
    for i in 0..=l1 - l2 {
        if &v[i..i + l2] == pat {
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