pub struct Card {
    pub suit: Suit,
    pub value: u8,
}

#[derive(Debug)]
pub enum Suit {
    Hearts, Spades, Clubs, Diamonds
}
