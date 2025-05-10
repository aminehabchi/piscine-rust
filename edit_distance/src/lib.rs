use std::collections::HashMap;
pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut x :usize=0;
    let mut source_map: HashMap<char,usize> = HashMap::new();
    let mut target_map: HashMap<char,usize> = HashMap::new();
    for ch in source.chars(){
        let val = source_map.entry(ch).or_insert(0);
        *val+=1;
    }

    for ch in target.chars(){
        let val = target_map.entry(ch).or_insert(0);
        *val+=1;
    }

    for (ch, val1) in &target_map {
        let val2=source_map.entry(*ch).or_insert(0);
        if *val1 != *val2{
            x+=((*val1 as i32 -*val2 as i32).abs()) as usize;
        }
    }  
    
    x
}