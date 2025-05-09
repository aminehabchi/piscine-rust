pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c,(c as f64).exp(),(c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut ss:String = String::new();
    for num in a.split(' '){
        let number: i32 = num.parse().expect("");
        let b=format!("{}",(number as f64).exp());
        ss.push_str(&b.to_owned());
        ss.push(' ');
    }
    let l=ss.len()-1;
    (a.to_owned(),ss[0..l].to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut a:Vec<f64>=Vec::<f64>::new();
    for n in &b {
        a.push((*n as f64).abs().ln());
    }

    (b.to_owned(),a)
}
