pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if prefix.len() >= s.len() {
        return None;
    }
    for _i in 0..prefix.len() {
        if s.chars().next() != prefix.chars().next() {
            return None;
        }
    }
    Some(&s[prefix.len()..])
}
