pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h/3.6
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
