fn n_space(n:usize)->String{
    let mut s:String=String::new();
    for _ in 0..n{
        s.push(' ');
    }
    s
}

pub fn get_diamond(c: char) -> Vec<String> {
    let mut d:Vec<String>=Vec::new();
    if c=='A'{
        d.push("A".to_string());
        return d;
    }
    
    let mut b:char='A';

    let mut diff:usize=1;

    while b<=c{
        let spaces:usize=c as usize-b as usize;

        let mut row:String=n_space(spaces);
        row.push(b);
        
        if b!='A'{

            row.push_str(&n_space(diff));
            diff+=2;
            row.push(b);
        }


        row.push_str(&n_space(spaces));

        d.push(row);
        b= ((b as u8) +1) as char;
    }

    let mut l :i32=d.len() as i32 -2 ;
    println!("{}",l);
    while l>=0{
        d.push(d[l as usize].clone());
        l-=1;
    }

    d
}