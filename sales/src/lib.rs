#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let mut a = 0.0;
        for p in &s.products {
            if *p.0 == ele {
                a = p.1;
                break;
            }
        }
        for p in &mut self.items {
            if *p.0 == ele {
                p.1 += a;
                return;
            }
        }
        self.items.push((ele, a))
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut r: Vec<f32> = vec![];
        for p in &self.items {
            r.push(p.1);
        }
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let total: f32 = r.iter().sum();
        let pers = r[0] / total;
        for i in 0..r.len() {
            r[i] = (r[i] * (1.0 - pers) * 100.0).round() / 100.0;
        }
        self.receipt = r;
        self.receipt.clone()
    }
}
