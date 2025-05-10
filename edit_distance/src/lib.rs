pub fn edit_distance(source: &str, target: &str) -> usize {
    backtrack(source.as_bytes(), target.as_bytes(), 0, 0)
}

fn backtrack(source: &[u8], target: &[u8], i: usize, j: usize) -> usize {
    if i == source.len() {
        return target.len() - j;
    }
    if j == target.len() {
        return source.len() - i; 
    }

    if source[i] == target[j] {
        backtrack(source, target, i + 1, j + 1) 
    } else {
        let insert = backtrack(source, target, i, j + 1); 
        let delete = backtrack(source, target, i + 1, j);    
        let replace = backtrack(source, target, i + 1, j + 1); 
        1 + insert.min(delete).min(replace)
    }
}