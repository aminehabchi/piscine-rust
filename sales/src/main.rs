use sales::*;

fn main() {
    let store = Store::new(
        vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12),
            (String::from("product D"), 9.75),
            (String::from("product E"), 1.75),
            (String::from("product F"), 23.75),
            (String::from("product G"), 2.75),
            (String::from("product H"), 1.64),
            (String::from("product I"), 15.23),
            (String::from("product J"), 2.1),
            (String::from("product K"), 54.91),
            (String::from("product L"), 43.99)
        ]
    );

    println!("{:?}", store);

    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));
    cart.insert_item(&store, String::from("product D"));
    cart.insert_item(&store, String::from("product E"));
    cart.insert_item(&store, String::from("product F"));
    cart.insert_item(&store, String::from("product G"));

    println!("{:?}", cart.generate_receipt());

    println!("{:?}", cart);
}
