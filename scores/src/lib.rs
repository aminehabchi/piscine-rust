

pub fn score(s :&str)->u64{
    let mut x:u64=0;
    
    let arr :Vec<&str>=["AEIOULNRST","DG","BCMP","FHVWY","K","JX","QZ"].to_vec();
    let score:Vec<u64>=[1,2,3,4,5,8,10].to_vec();
    for ch in s.chars(){
        for i in 0..arr.len(){
            if arr[i].contains(ch.to_uppercase().next().unwrap()){
                x+=score[i];
                break;
            }
        }
    }


    x 
}