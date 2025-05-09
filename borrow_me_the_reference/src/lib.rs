// "bpp--o+er+++sskroi-++lcw"
pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
   
    while i < s.len() {
        let ch = s.chars().nth(i).unwrap();
        if ch=='-' && i>0{
            s.remove(i);
            if s.len()==0{
                break
            }
            s.remove(i-1);
            if i>0{
                i-=1;
            }
        }else{
            i+=1;
        }
    }  

    i=s.len()-1;
    while i > 0 {
        let ch = s.chars().nth(i).unwrap();
        if ch=='+'{
            s.remove(i);
            if s.len()==0{
                break
            }
            s.remove(i);
        }
        i-=1;
    }  
}

fn calcul(s :String,opr :char)->String{
    let parts: Vec<&str> = s.split(opr).collect();
    let n1:i32=parts[0].parse().expect("");
    let n2:i32=parts[1].parse().expect("");

    if opr=='-'{
        return (n1-n2).to_string();
    }
    (n1+n2).to_string()
}

pub fn do_operations(v: &mut [String]) {
    for seq in v{
        if seq.contains('-'){
            *seq=calcul(seq.to_string(),'-');
        }else if seq.contains('+'){
            *seq=calcul(seq.to_string(),'+');
        }
    }
}