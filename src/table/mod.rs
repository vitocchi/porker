mod card;
use card::CardSet;
use std::fmt;

pub struct Table {
    deck: CardSet,
    hand: CardSet
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Deck")?;
        writeln!(f, "{}", self.deck)?;

        writeln!(f, "Hand")?;
        writeln!(f, "{}", self.hand)
    }
}

impl Table {
    pub fn new() -> Table {
        Table { deck: CardSet::new_full(), hand: CardSet::new() }
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    pub fn pick_from_deck(&mut self) -> Result<(), String> {
        match self.deck.pick_card() {
            Some(card) => {
                self.hand.add_card(card);
                Ok(())
            },
            None => {
                Err("deck is out".to_string())
            }
        }
    }
}