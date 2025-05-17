use logic_number::*;

fn main() {
    let array = [9, 10, 153, 154];
    for pat in &array {
  
            println!(
                " {}  {} ",
                number_logic(*pat),
                pat
            )
        
   
    }
}