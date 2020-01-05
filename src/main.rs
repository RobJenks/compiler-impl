use itertools::Itertools;
use lexer;
use lexer::Lexer;

fn main() {
    let lexer = Lexer::new(true);
    println!("{}", lexer.tokenize("let myVar = 2 * 3 +    6")
        .data()
        .iter()
        .map(|t| t.to_string())
        .join(","));
}
