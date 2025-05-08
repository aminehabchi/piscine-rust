/*
Formula	
(33.8°F − 32) × 5/9 = 1°C
*/
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0)/1.8
}

/*
Formula	
(c × 9/5) + 32 = f
*/

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
     (c * 1.8) + 32.0 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
        assert_eq!(2+2, 4);
    }
}
