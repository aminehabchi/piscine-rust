use expected_variable::*;

fn main() {
    let cases = vec![
        ("On_Point", "on_point"),
        ("aaa", "bbb"),
        ("something", "something_completely_different"),
        ("BenedictCumberbatch", "BeneficialCucumbersnatch"),
    ];

    for (a, b) in cases {
        match expected_variable(a, b) {
            Some(score) => println!("{score} close to it"),
            None => println!("Not similar enough"),
        }
    }
}