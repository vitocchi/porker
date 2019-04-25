mod cardset;
use cardset::CardSet;
use std::fmt;
use std::collections::HashMap;

const HAND_NUMBER: u8 = 5;

#[derive(PartialEq)]
pub enum Value {
    StraightFlush,
    Straight,
    Flush,
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
            Value::StraightFlush => "!! straight flush !!",
            Value::Straight => "! straight !",
            Value::Flush => "! flush !",
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
    pub hand: CardSet,
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

    pub fn get_variant(&self) -> Result<Value, String> {
        if self.hand.get_number() != HAND_NUMBER {
            return Err("number of hand is not 5!".to_string());
        }
        let mut groups = HashMap::new();
        let CardSet(vec) =  &self.hand;
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
                let mut min = 1;
                let mut max = 13;
                let suit = &vec[0].suit;
                let mut is_flush = true;
                let mut is_straigt = true;
                for card in vec {
                    if min > card.number {
                        min = card.number;
                    }
                    if max < card.number {
                        max = card.number;
                    }
                    if *suit != card.suit {
                        is_flush = false;
                    }
                }
                if max - min != 4 {
                    is_straigt = false;
                } 

                match (is_straigt, is_flush) {
                    (true, true) => return Ok(Value::StraightFlush),
                    (true, false) => return Ok(Value::Straight),
                    (false, true) => return Ok(Value::Flush),
                    (false, false) => return Ok(Value::HighCard)
                };
            },
            _ => {
                return Err("error".to_string());
            }
        }
    }
}
