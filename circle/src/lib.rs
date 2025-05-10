use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center :Point,
    pub radius :f64,
}

impl Circle {
    pub fn new(x :f64,y :f64,radius :f64)->Circle{
        let circl:Circle=Circle{
            center:Point(x,y),
            radius:radius,
        };
        circl
    }
    pub fn diameter(&self)->f64{
        self.radius*2.0
    }
    pub fn area(&self)->f64{
        PI*self.radius*self.radius
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        let d: f64 = self.center.distance(other.center);
        
        // Check if |r1 - r2| < d < r1 + r2
        (self.radius - other.radius).abs() < d && d < (self.radius + other.radius)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64,pub f64);

impl Point {
    // d=√((x2 – x1)² + (y2 – y1)²)
 pub fn distance(&self, other: Point)->f64{
        ((other.0-self.0).powf(2.0)+(other.1-self.1).powf(2.0)).sqrt()
    }
}