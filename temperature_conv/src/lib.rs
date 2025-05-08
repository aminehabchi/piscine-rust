/*
Formula	
(33.8°F − 32) × 5/9 = 1°C
*/

fn to_fixed2(n: f64) -> f64{
    (n * 100.0).round() / 100.0
}
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    to_fixed2((f - 32.0)*(1.0/(9.0/5.0)))
}

/*
Formula	
(c × 9/5) + 32 = f
*/

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    to_fixed2( c * (9.0/5.0) + 32.0 )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
        assert_eq!(2+2, 4);
    }
}
