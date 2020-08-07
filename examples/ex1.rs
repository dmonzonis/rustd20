use rustd20::dice;
use rustd20::parser;

fn main() {
    // Rolling dice
    println!("Rolling a 3d6: {}", dice::roll(6, 3));
    // Parsing dice rolls
    let s = "1d20,1d8,2d12+5";
    println!("Rolling {}: {}", s, parser::parse_roll(s));
}
