mod cardset;
use cardset::CardSet;
use std::fmt;
use std::collections::HashMap;

const HAND_NUMBER: u8 = 5;

#[derive(PartialEq)]
pub enum Value {
    FourOfAKind,
    ThreeOfAKind,
    FullHouse,
    TwoPair,
    OnePair,
    HighCard,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Value::FourOfAKind => "four of kind",
            Value::ThreeOfAKind => "thee of kind",
            Value::FullHouse => "full house",
            Value::TwoPair => "two pair",
            Value::OnePair => "one pair",
            Value::HighCard => "high card"
        };
        writeln!(f, "{}", s)
    }
}

pub struct Table {
    deck: CardSet,
    hand: CardSet,
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
        Table {
            deck: CardSet::new_full(),
            hand: CardSet::new(),
        }
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle();
    }

    pub fn init_hand(&mut self) -> Result<(), String> {
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
            }
            None => Err("deck is out".to_string()),
        }
    }

    pub fn get_variant(self) -> Result<Value, String> {
        if self.hand.get_number() != HAND_NUMBER {
            return Err("number of hand is not 5!".to_string());
        }
        let mut groups = HashMap::new();
        let CardSet(vec) =  self.hand;
        for card in vec {
            let count = groups.entry(card.number).or_insert(0);
            *count += 1;
        }
        match groups.len() {
            2 => {
                for (_, count) in groups.iter() {
                    if *count == 4 {
                        return Ok(Value::FourOfAKind)
                    }
                }
                return Ok(Value::FullHouse);
            },
            3 => {
                for (_, count) in groups.iter() {
                    if *count == 3 {
                        return Ok(Value::ThreeOfAKind)
                    }
                }
                return Ok(Value::TwoPair);
            },
            4 => {
                return Ok(Value::OnePair);
            },
            5 => {
                return Ok(Value::HighCard);
            },
            _ => {
                return Err("error".to_string());
            }
        }
    }
}
