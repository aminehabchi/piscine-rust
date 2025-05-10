use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum:i32=list.iter().sum();
    sum as f64/list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    if list.len()==0{
        return 0;
    }
    let mut l=list.to_vec();
    l.sort();

    if l.len()%2==1{
        return l[l.len()/2];
    }
    let m=l[l.len()/2]+l[(l.len()/2) -1];
    m/2
}

pub fn mode(list: &[i32]) -> i32 {
    if list.len()==0{
        return 0
    }
    let mut m:HashMap<i32,usize> = HashMap::new();
    
    for n in list{
        let value=m.entry(*n).or_insert(0);
        *value+=1;
    }    
    let mut max:usize=1;
    let mut modd:i32=list[0];
    
    for (key,value) in m{
        if value>max{
            max=value;
            modd=key;
        }
    }

    
    modd
}