use rand::Rng;

// Suit (Heart, Diamond, Spade or Club).
#[derive(Debug,PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// Rank Ace, King, Queen or Jack.
#[derive(Debug,PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(0..4);
        match x{
            0=> return Suit::Hearts,
            1=> return Suit::Diamonds,
            2=> return Suit::Clubs,
            3=> return Suit::Spades,
            _ => Suit::Spades,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value{
            0=> return Suit::Hearts,
            1=> return Suit::Diamonds,
            2=> return Suit::Clubs,
            3=> return Suit::Spades,
            _ => Suit::Spades,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let x: i32 = rng.gen_range(0..13);
        match x{
            0=> return Rank::Ace,
            1=> return Rank::Two,
            2=> return Rank::Three,
            3=> return Rank::Four,
            4=> return Rank::Five,
            5=> return Rank::Six,
            6=> return Rank::Seven,
            7=> return Rank::Eight,
            8=> return Rank::Nine,
            9=> return Rank::Ten,
            10=> return Rank::Jack,
            11=> return Rank::Queen,
            12=> return Rank::King,
            _=> return Rank::Ace,
        }
        
    }

    pub fn translate(value: u8) -> Rank {
        match value{
            0=> return Rank::Ace,
            1=> return Rank::Two,
            2=> return Rank::Three,
            3=> return Rank::Four,
            4=> return Rank::Five,
            5=> return Rank::Six,
            6=> return Rank::Seven,
            7=> return Rank::Eight,
            8=> return Rank::Nine,
            9=> return Rank::Ten,
            10=> return Rank::Jack,
            11=> return Rank::Queen,
            12=> return Rank::King,
            _ => return Rank::Ace,
        }

    }
}
#[derive(Debug,PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    let winner_card=Card{
        suit: Suit::Spades,
        rank: Rank::Ace,
    };
    *card==winner_card
}