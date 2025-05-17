pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    let mut j: usize = 0;
    if text.find("qu") == Some(1) {
        if let Some(first_char) = text.chars().next() {
            if !vowels.contains(first_char) {
                let (l, r) = text.split_at(3);
                let mut s = String::new();
                s.push_str(r);
                s.push_str(l);
                s.push_str("ay");
                return s;
            }
        }
    }

    for (i, ch) in text.chars().enumerate() {
        if vowels.contains(ch) {
            j = i;
            break;
        }
    }

    let (l, r) = text.split_at(j);
    let mut s = String::new();
    s.push_str(r);
    s.push_str(l);
    s.push_str("ay");
    s
}
