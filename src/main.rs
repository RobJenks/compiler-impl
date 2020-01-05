use itertools::Itertools;
use lexer;
use lexer::Lexer;

fn main() {
    let lexer = Lexer::new(true);
    println!("{}", lexer.tokenize("1 -23    4   +   \n 5")
        .data()
        .iter()
        .map(|t| t.to_string())
        .join(","));
}
