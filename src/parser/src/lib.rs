use common::tokens::token::Token;
use common::nodes::{AstNode, AstExpression, AstLiteral};
use common::tokens::Tokenized;

mod production;

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self { Self {} }

    pub fn parse(&self, tokens: Tokenized) -> AstNode {
        //AstNode::Expression(AstExpression::Literal(AstLiteral::Integral(12)))
        AstNode::Statement(production::parse_statement(tokens.data().iter().peekable()).result().clone().unwrap())
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
