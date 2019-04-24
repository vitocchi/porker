mod card;
use card::Card;
use std::fmt;

pub struct Table {
    deck: Deck
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Deck")?;
        write!(f, "{}", self.deck)
    }
}

impl Table {
    pub fn new() -> Table {
        Table{deck: Deck::new()}
    }
}

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Deck {
        Deck{cards: card::generate_set()}
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            writeln!(f, "{}", card)?;
        }
        writeln!(f, "")
    }
}