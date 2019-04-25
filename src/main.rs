extern crate porkerlib;

use porkerlib::table;
fn main() {
    let mut table = table::Table::new();
    println!("{}", table);
    table.shuffle_deck();
    println!("{}", table);
}
