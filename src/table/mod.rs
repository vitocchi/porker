mod card;
use card::Card;
use card::CardSet;
use std::fmt;

pub struct Table {
    deck: CardSet,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Deck")?;
        write!(f, "{}", self.deck)
    }
}

impl Table {
    pub fn new() -> Table {
        Table { deck: CardSet::new() }
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }
}