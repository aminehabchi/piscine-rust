use json::{object};

pub struct Food {
   pub name:String,
   pub calories:(String,String),
   pub fats:f64,
   pub carbs:f64,
   pub proteins:f64,
   pub nbr_of_portions:f64,
}

fn round_two_decimals(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut fats:f64=0.0;
    let mut carbs:f64=0.0;
    let mut proteins:f64=0.0;
    let mut cals:f64=0.0;

   for food in foods{
    let (n,u)=food.calories.1.split_at(food.calories.1.len()-4);
    cals=cals+n.parse::<f64>().unwrap_or(0.0)*food.nbr_of_portions;

    fats=fats+food.fats*food.nbr_of_portions;
    carbs=carbs+food.carbs*food.nbr_of_portions;
    proteins=proteins+food.proteins*food.nbr_of_portions;
   }


   object! {
            cals:round_two_decimals(cals),
            carbs:round_two_decimals(carbs),
            proteins:round_two_decimals(proteins),
            fats:round_two_decimals(fats),
        }
}