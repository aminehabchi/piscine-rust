pub fn number_logic(num: u32) -> bool {
    let s:String=num.to_string();
    let mut x:u32=0;

    let l:u32=s.len() as u32;
    
    for n in s.bytes(){
        let nn:u32=n as u32 -48;
        x+=nn.pow(l);

    }

    x==num
}