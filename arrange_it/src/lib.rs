
fn remove_and_return(s:&mut String)->i32{
    for (i,ch) in s.chars().enumerate(){
        if ch>='0'&& ch<='9'{
            s.remove(i);
            return ch.to_digit(10).unwrap() as i32;
        }
    }
    0
}

pub fn arrange_phrase(phrase: &str) -> String {
    let iter: Vec<String> = phrase
        .split(' ')
        .map(String::from)
        .collect();

    let mut arr: Vec<String> = vec![String::new(); iter.len()];

    for s in &iter {
        let mut ss = s.to_owned();
        let i = remove_and_return(&mut ss);
        arr[(i as usize) - 1] = ss;
    }

    arr.join(" ")
}