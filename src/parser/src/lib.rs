use common::tokens::Tokenized;
use common::nodes::*;

mod production;

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self { Self {} }

    pub fn parse(&self, tokens: Tokenized) -> AstNode {
        AstNode::Statement(production::parse_statement(tokens.data()
            .iter()
            .peekable())
            .result()
            .clone()
            .unwrap())
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
