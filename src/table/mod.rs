mod card;
use card::Card;
use card::suit::Suit;
use std::fmt;

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck{cards: Vec::new()};
        for number in card::MIN_NUMBER..card::MAX_NUMBER + 1 {
            deck.cards.push(Card::new(Suit::Spade, number).unwrap());
            deck.cards.push(Card::new(Suit::Diamond, number).unwrap());
            deck.cards.push(Card::new(Suit::Club, number).unwrap());
            deck.cards.push(Card::new(Suit::Heart, number).unwrap());
        }
        deck
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            writeln!(f, "{}", card);
        }
        writeln!(f, "")
    }
}