pub fn rotate(input: &str, key: i8) -> String {
    let mut s:String=String::new();

    for b in input.bytes(){
        if (b as char).is_alphabetic(){
            let mut base='a';
            if (b as u8) < 96{
                base='A';
            }

            let mut letter:i8=b as i8 -base as i8 +key ;
            if letter<0{
                letter+=26;
            }

            letter%=26;
            s.push((letter as u8 + base as u8) as char);


        }else{
            s.push(b as char);
        }    

    }

    s
}