use itertools::Itertools;
use common::nodes::*;
use lexer;
use lexer::Lexer;
use parser::Parser;
use common::nodes::render::RenderableNode;

fn main() {
    env_logger::init();

    let lexer = Lexer::new(true);
    let tokens = lexer.tokenize("let myVar = 2 * 3 +    6");

    println!("{}", tokens
        .data()
        .iter()
        .map(|t| t.to_string())
        .join(","));

    let parser = Parser::new();
    let ast: AstNode = parser.parse(tokens);

    println!("{:?}", ast);

    println!("{}", ast.render(0));
}
