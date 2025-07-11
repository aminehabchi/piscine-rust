#[allow(dead_code)]
pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

impl Vehicle for Truck<'_> {
    fn model(&self) -> &str {
        &self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

impl Vehicle for Car<'_> {
    fn model(&self) -> &str {
        &self.model
    }
    fn year(&self) -> u32 {
        self.year
    }
}

pub fn all_models<'a>(list: Vec<&'a dyn Vehicle>) -> Vec<&'a str> {
    let mut ar: Vec<&'a str> = vec![];
    for a in list {
        ar.push(a.model());
    }
    ar
}
