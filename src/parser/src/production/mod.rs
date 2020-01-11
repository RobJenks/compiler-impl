mod result;
use common::tokens::token::Token;
use common::nodes::*;
use common::tokens::TokenIter;
use common::nodes::parsing::NodeParseResult;
use std::any::Any;
use common::util::flatten::Flatten;

//pub fn parse_root(tokens: TokenIter) -> NodeParseResult<AstNode> {
//    if let Some(statement) = parse_statement(tokens.clone()) {
//        Ok(AstNode::Statement(statement))
//    }
//    else if let Some(expr) = parse_expression(tokens.clone()) {
//        Ok(AstNode::Expression(expr))
//    }
//    else {
//        Err("Could not parse root AST node; no token match".to_string())
//    }
//}

fn parse_unary_operator(tokens: TokenIter) -> NodeParseResult<AstUnaryOperator> {
    let mut it = tokens.clone();

    match it.next() {
        Some(Token::Plus) => NodeParseResult::of(AstUnaryOperator::Plus, it),
        Some(Token::Minus) => NodeParseResult::of(AstUnaryOperator::Minus, it),
        _ => NodeParseResult::none()
    }
}

fn parse_binary_operator(tokens: TokenIter) -> NodeParseResult<AstBinaryOperator> {
    let mut it = tokens.clone();

    match it.next() {
        Some(Token::Plus) => NodeParseResult::of(AstBinaryOperator::Add, it),
        Some(Token::Minus) => NodeParseResult::of(AstBinaryOperator::Minus, it),
        Some(Token::Asterisk) => NodeParseResult::of(AstBinaryOperator::Multiply, it),
        Some(Token::FSlash) => NodeParseResult::of(AstBinaryOperator::Divide, it),
        Some(Token::Equals) => NodeParseResult::of(AstBinaryOperator::Equals, it),
        _ => NodeParseResult::none()
    }
}

fn parse_identifier(tokens: TokenIter) -> NodeParseResult<AstIdentifier> {
    let mut it = tokens.clone();
    println!("ident: {:?}", it.peek());
    match it.next() {
        Some(Token::Let) => NodeParseResult::of(AstIdentifier::Let, it),
        Some(Token::Identifier(x)) => NodeParseResult::of(AstIdentifier::Variable(x.clone()), it),
        _ => NodeParseResult::none()
    }
}

fn parse_literal(tokens: TokenIter) -> NodeParseResult<AstLiteral> {
    let mut it = tokens.clone();

    match it.next() {
        Some(Token::Integral(x)) => NodeParseResult::of(AstLiteral::Integral(*x), it),
        _ => NodeParseResult::none()
    }
}

pub fn parse_statement(tokens: TokenIter) -> NodeParseResult<AstStatement> {
    [
        parse_let_assignment
    ]
        .iter().map(|f| f(tokens.clone()))
        .next().unwrap_or_else(|| NodeParseResult::none())
}

fn parse_let_assignment(tokens: TokenIter) -> NodeParseResult<AstStatement> {
    let mut it = tokens.clone();

    if let (Some((((id_let, id), eq),expr)), next) =
        parse_identifier(it)        // let
            .and(parse_identifier)      // <var>
            .and(parse_binary_operator) // =
            .and(parse_expression)      // <expr>
        .output() {

        match (id_let, id, eq, expr) {
            (AstIdentifier::Let, var @ AstIdentifier::Variable(_),AstBinaryOperator::Equals, expr)
                => NodeParseResult::of_opt(AstStatement::Assignment(var, expr), next),

            _ => NodeParseResult::none()
        }

    }
    else { NodeParseResult::none() }
}


fn parse_expression(tokens: TokenIter) -> NodeParseResult<AstExpression> {
    [
        parse_literal_expression
    ]
        .iter().map(|f| f(tokens.clone()))
        .next().unwrap_or_else(|| NodeParseResult::none())
}

fn parse_literal_expression(tokens: TokenIter) -> NodeParseResult<AstExpression> {
    if let (Some(literal), it) = parse_literal(tokens).output() {
        NodeParseResult::of_opt(AstExpression::Literal(literal), it)
    }
    else { NodeParseResult::none() }
}