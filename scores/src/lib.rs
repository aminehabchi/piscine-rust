

pub fn score(s :&str)->u64{
    let mut x:usize=0;
    
    let arr :Vec<&str>=["AEIOULNRST","DG","BCMP","FHVWY","K","JX","QZ"].to_vec();

    for ch in s.chars(){
        for i in 0..arr.len(){
            if arr[i].contains(ch.to_uppercase().next().unwrap()){
                x+=i+1;
                break;
            }
        }
    }


    x as u64
}