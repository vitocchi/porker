mod card;
use card::CardSet;
use std::fmt;

const HAND_NUMBER: u8 = 5;

pub struct Hand (CardSet);

pub enum Value {
    HighCard
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "highcard")
    }
}

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

    pub fn init_hand(&mut self) -> Result<(), String>{
        for _ in 0..HAND_NUMBER {
            self.pick_from_deck()?
        }
        Ok(())
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

    pub fn get_variant(self) -> Result<Value, String> {
        if self.hand.get_number() != HAND_NUMBER {
            return Err("number of hand is not 5!".to_string())
        }
        Ok(Value::HighCard)
    }
}