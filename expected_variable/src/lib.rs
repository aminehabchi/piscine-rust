use std::borrow::Cow;

/// Your edit_distance function must be implemented already.
/// Here's a simple version if you don't have it:
fn edit_distance(a: &str, b: &str) -> usize {
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        for j in 0..=b.len() {
            dp[i][j] = if i == 0 {
                j
            } else if j == 0 {
                i
            } else if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[i - 1][j - 1]
            } else {
                1 + std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i][j - 1], dp[i - 1][j]))
            };
        }
    }
    dp[a.len()][b.len()]
}

fn is_snake_case(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase() || c == '_')
}

pub fn expected_variable(a: &str, b: &str) -> Option<String> {
    if a.contains(' ') || b.contains(' ') {
        return None;
    }
    let aa = a.to_lowercase();
    let bb = b.to_lowercase();
    if is_snake_case(&aa) && is_snake_case(&bb) {
        return None;
    }

    let distance = edit_distance(&aa, &bb);
    let similarity = 1.0 - (distance as f64 / b.len() as f64);
    let res = (similarity * 100.0).round();
    
    if res > 50.0 {
        return Some(format!("{}%", res));
    }

    None
}