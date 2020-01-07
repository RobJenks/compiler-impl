pub type NodeId = usize;

pub enum AstLiteral {
    Integral
}

pub enum AstIdentifier {
    Variable
}

pub enum AstUnaryOperator {
    Plus,
    Minus
}

pub enum AstBinaryOperator {
    Add,
    Minus,
    Multiply,
    Divide
}

pub enum AstExpression {
    Literal(AstLiteral),
    Identifier(AstIdentifier),
    Unary(AstUnaryOperator, Box<AstExpression>),
    Binary(Box<AstExpression>, AstBinaryOperator, Box<AstExpression>),
}

pub enum AstStatement {
    Assignment(AstIdentifier, Box<AstExpression>)
}

pub enum AstNode {
    AstExpression,
    AstStatement
}
