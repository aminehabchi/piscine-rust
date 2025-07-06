use std::collections::HashMap;
use std::hash::Hash;
pub fn slices_to_map<'a, T, U>(a: &'a [T], b: &'a [U]) -> HashMap<&'a T, &'a U> where T: Eq, T: Hash {
    let mut m: HashMap<&'a T, &'a U> = HashMap::new();
    let l = a.len().min(b.len());
    for i in 0..l {
        m.insert(&a[i], &b[i]);
    }
    m
}
