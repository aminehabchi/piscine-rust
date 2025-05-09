pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    
    for name in &names {
        let parts: Vec<&str> = name.split(' ').collect();

        if parts.len() >= 2 {
            let mut s = String::new();
            if let Some(c1) = parts[0].chars().next() {
                s.push(c1);
                s.push('.');
                s.push(' ');
            }
            if let Some(c2) = parts[1].chars().next() {
                s.push(c2);
                s.push('.');
            }
            arr.push(s);
        }
    }

    arr
}