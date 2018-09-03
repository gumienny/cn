extern crate cn;

use cn::cli;

fn main() {
    let options = cli::Options::load();

    println!("{:#?}", options);
}
