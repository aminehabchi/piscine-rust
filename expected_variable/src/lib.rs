pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();
    let source = source.as_bytes();
    let target = target.as_bytes();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if source[i - 1] == target[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]);
            }
        }
    }

    dp[m][n]
}


pub fn expected_variable(a: &str, b: &str) -> Option<String> {
    if b.is_empty() {
        return None;
    }

    let a1 = a.to_lowercase();
    let b1 = b.to_lowercase();

    let distance = edit_distance(&a1, &b1);
    let similarity = ((b.len() - distance) as f64 * 100.0) / b.len() as f64;
    let percentage = similarity.round() as i32;

    if percentage <= 50 {
        None
    } else {
        Some(format!("{}%", percentage))
    }
}