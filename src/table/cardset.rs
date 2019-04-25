mod card;
mod suit;
use card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use suit::Suit;

pub struct CardSet(Vec<Card>);

impl fmt::Display for CardSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in {
            let CardSet(vec) = self;
            vec
        } {
            writeln!(f, "{}", card)?;
        }
        writeln!(f, "")
    }
}

impl CardSet {
    pub fn new_full() -> CardSet {
        let mut vec = Vec::<Card>::new();
        for number in card::MIN_NUMBER..card::MAX_NUMBER + 1 {
            vec.push(Card::new(Suit::Spade, number).unwrap());
            vec.push(Card::new(Suit::Diamond, number).unwrap());
            vec.push(Card::new(Suit::Club, number).unwrap());
            vec.push(Card::new(Suit::Heart, number).unwrap());
        }
        CardSet(vec)
    }
    pub fn new() -> CardSet {
        CardSet(Vec::<Card>::new())
    }

    pub fn shuffle(&mut self) {
        let CardSet(vec) = self;
        vec.shuffle(&mut thread_rng());
    }

    pub fn pick_card(&mut self) -> Option<Card> {
        let CardSet(vec) = self;
        vec.pop()
    }

    pub fn add_card(&mut self, card: Card) {
        let CardSet(vec) = self;
        vec.push(card)
    }

    pub fn get_number(self) -> u8 {
        let CardSet(vec) = self;
        vec.len() as u8
    }
}
