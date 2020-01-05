use itertools::Itertools;
use lexer;

fn main() {
    println!("{}", lexer::Lexer::tokenize("1 -23    4   +   \n 5")
        .data()
        .iter()
        .map(|t| t.to_string())
        .join(","));
}
