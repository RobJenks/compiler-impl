use itertools::Itertools;
use lexer;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let lexer = Lexer::new(true);
    let tokens = lexer.tokenize("let myVar = 2 * 3 +    6");

    println!("{}", tokens
        .data()
        .iter()
        .map(|t| t.to_string())
        .join(","));

    let parser = Parser::new();
    let ast = parser.parse(tokens);

    println!("{:?}", ast);
}
