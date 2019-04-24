extern crate porkerlib;

use porkerlib::Card;
use porkerlib::Suit;
fn main() {
    println!("Hello, world!");
    let card = Card::new(Suit::Club, 2).unwrap();
    println!("{:?}", card);
}