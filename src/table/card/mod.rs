mod suit;
use std::fmt;
use suit::Suit;
use rand::thread_rng;
use rand::seq::SliceRandom;

// トランプのカード
const MIN_NUMBER: u8 = 1;
const MAX_NUMBER: u8 = 13;

pub struct CardSet (Vec<Card>);

impl fmt::Display for CardSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in { let CardSet(vec) = self; vec }  {
            writeln!(f, "{}", card)?;
        }
        writeln!(f, "")
    }
}

impl CardSet {
    pub fn new_full () -> CardSet {
        let mut vec = Vec::<Card>::new();
        for number in MIN_NUMBER..MAX_NUMBER + 1 {
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
}

pub struct Card {
    suit: Suit, // マーク
    number: u8, //数字
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} : {1: >2}", self.suit, self.number)
    }
}
impl Card {
    pub fn new(suit: Suit, number: u8) -> Result<Card, String> {
        Card::check_number(number)?;
        Ok(Card { suit, number })
    }

    fn check_number(number: u8) -> Result<(), String> {
        if number > MAX_NUMBER || number < MIN_NUMBER {
            return Err(number.to_string() + " is to large as number of card!!");
        }
        return Ok(());
    }
}
