pub mod parsing;

pub type NodeId = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Literal(AstLiteral),
    Identifier(AstIdentifier),
    UnaryOperator(AstUnaryOperator),
    BinaryOperator(AstBinaryOperator),
    Expression(AstExpression),
    Statement(AstStatement)
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstLiteral {
    Integral(i64),
    String(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstIdentifier {
    Let,
    Variable(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstUnaryOperator {
    Plus,
    Minus
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstBinaryOperator {
    Add,
    Minus,
    Multiply,
    Divide,
    Equals
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstExpression {
    Literal(AstLiteral),
    Identifier(AstIdentifier),
    Unary(AstUnaryOperator, Box<AstExpression>),
    Binary(Box<AstExpression>, AstBinaryOperator, Box<AstExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstStatement {
    Assignment(AstIdentifier, AstExpression)
}
