use log::debug;
use common::tokens::token::Token;
use common::nodes::*;
use common::tokens::TokenIter;
use common::nodes::parsing::NodeParseResult;

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
    debug_prod("unary-op", &it);

    match it.next() {
        Some(Token::Plus) => NodeParseResult::of(AstUnaryOperator::Plus, it),
        Some(Token::Minus) => NodeParseResult::of(AstUnaryOperator::Minus, it),
        _ => NodeParseResult::none()
    }
}

fn parse_binary_operator(tokens: TokenIter) -> NodeParseResult<AstBinaryOperator> {
    let mut it = tokens.clone();
    debug_prod("bin-op", &it);

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
    debug_prod("ident", &it);

    match it.next() {
        Some(Token::Let) => NodeParseResult::of(AstIdentifier::Let, it),
        Some(Token::Identifier(x)) => NodeParseResult::of(AstIdentifier::Variable(x.clone()), it),
        _ => NodeParseResult::none()
    }
}

fn parse_literal(tokens: TokenIter) -> NodeParseResult<AstLiteral> {
    let mut it = tokens.clone();
    debug_prod("literal", &it);

    match it.next() {
        Some(Token::Integral(x)) => NodeParseResult::of(AstLiteral::Integral(*x), it),
        _ => NodeParseResult::none()
    }
}

pub fn parse_statement(tokens: TokenIter) -> NodeParseResult<AstStatement> {
    debug_prod("stmnt", &tokens);

    [
        parse_let_assignment
    ]
        .iter().map(|f| f(tokens.clone()))
        .next().unwrap_or_else(|| NodeParseResult::none())
}

fn parse_let_assignment(tokens: TokenIter) -> NodeParseResult<AstStatement> {
    debug_prod("let-assn", &tokens);

    if let (Some((((id_let, id), eq), expr)), next) =
        parse_identifier(tokens)        // let
            .and(parse_identifier)      // <var>
            .and(parse_binary_operator) // =
            .and(parse_expression)      // <expr>
        .output() {

        match (id_let, id, eq, expr) {
            (AstIdentifier::Let, var @ AstIdentifier::Variable(_), AstBinaryOperator::Equals, expr)
                => NodeParseResult::of_opt(AstStatement::Assignment(var, expr), next),

            _ => NodeParseResult::none()
        }

    }
    else { NodeParseResult::none() }
}


fn parse_expression(tokens: TokenIter) -> NodeParseResult<AstExpression> {
    debug_prod("expr", &tokens);

    [
        parse_unary_expression,
        parse_literal_expression,
        parse_identifier_expression
    ]
        .iter().map(|f| f(tokens.clone()))
        .filter(NodeParseResult::<AstExpression>::is_valid)
        .next().unwrap_or_else(|| NodeParseResult::none())
}

fn parse_expression_prime(tokens: TokenIter) -> NodeParseResult<AstExpressionPrime> {
    debug_prod("expr'", &tokens);

    match [
            parse_binary_expression
        ]
            .iter().map(|f| f(tokens.clone()))
            .filter(NodeParseResult::<AstExpressionPrime>::is_valid)
            .next().unwrap_or_else(|| NodeParseResult::none()).output() {

        (Some(e_prime),next) => NodeParseResult::of_opt(e_prime, next),
        _ => NodeParseResult::of(AstExpressionPrime::EPS, tokens.clone())
    }
}


fn parse_literal_expression(tokens: TokenIter) -> NodeParseResult<AstExpression> {
    debug_prod("lit-expr", &tokens);

    if let (Some((literal, e_prime)), it) =
        parse_literal(tokens).and(parse_expression_prime).output() {

        NodeParseResult::of_opt(AstExpression::Literal(literal, Box::new(e_prime)), it)
    }
    else { NodeParseResult::none() }
}

fn parse_identifier_expression(tokens: TokenIter) -> NodeParseResult<AstExpression> {
    debug_prod("id-expr", &tokens);

    if let (Some((id, e_prime)), it) =
        parse_identifier(tokens).and(parse_expression_prime).output() {

        NodeParseResult::of_opt(AstExpression::Identifier(id, Box::new(e_prime)), it)
    }
    else { NodeParseResult::none() }
}

fn parse_unary_expression(tokens: TokenIter) -> NodeParseResult<AstExpression> {
    debug_prod("una-expr", &tokens);

    if let (Some(((unary, expr), e_prime)), next) =
        parse_unary_operator(tokens).and(parse_expression).and(parse_expression_prime).output() {

        NodeParseResult::of_opt(AstExpression::Unary(unary, Box::new(expr), Box::new(e_prime)), next)
    }
    else { NodeParseResult::none() }
}

fn parse_binary_expression(tokens: TokenIter) -> NodeParseResult<AstExpressionPrime> {
    debug_prod("bin-exp", &tokens);

    if let (Some(((binary, rhs), e_prime)), next) =
        parse_binary_operator(tokens).and(parse_expression).and(parse_expression_prime).output() {

        NodeParseResult::of_opt(AstExpressionPrime::Binary(binary, Box::new(rhs), Box::new(e_prime)), next)
    }
    else { NodeParseResult::none() }
}

fn debug_prod(production_type: &str, stream: &TokenIter) {
    debug!("Parse {: <8}: {:?}", production_type, stream);
}
