extern crate porkerlib;

use porkerlib::table::Deck;
fn main() {
    println!("Hello, world!");
    let deck = Deck::new();
    println!("{}", deck);
}