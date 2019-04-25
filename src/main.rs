extern crate porkerlib;

use porkerlib::table;
fn main() {
    let mut table = table::Table::new();
    table.shuffle_deck();
    println!("{}", table);
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();
    println!("{}", table);
}
