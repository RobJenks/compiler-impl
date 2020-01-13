use crate::nodes::{AstStatement, AstIdentifier, AstLiteral, AstUnaryOperator, AstBinaryOperator, AstExpression, AstExpressionPrime, AstNode};

pub trait RenderableNode {
    fn render(&self, level: usize) -> String;
}

impl RenderableNode for AstNode {
    fn render(&self, level: usize) -> String {
        match self {
            AstNode::Identifier(n) => format!("{}node [identifier]\n{}", indent(level), n.render(level+1)),
            AstNode::Literal(n) => format!("{}node [literal]\n{}", indent(level), n.render(level+1)),
            AstNode::UnaryOperator(n) => format!("{}node [unary operator]\n{}", indent(level), n.render(level+1)),
            AstNode::BinaryOperator(n) => format!("{}node [binary operator]\n{}", indent(level), n.render(level+1)),
            AstNode::Expression(n) => format!("{}node [expression]\n{}", indent(level), n.render(level+1)),
            AstNode::Statement(n) => format!("{}node [statement]\n{}", indent(level), n.render(level+1))
        }
    }
}

impl RenderableNode for AstLiteral {
    fn render(&self, level: usize) -> String {
        match self {
            AstLiteral::Integral(n) => format!("{}literal: {} [i64]", indent(level), n),
            AstLiteral::String(s) => format!("{}literal: {} [string]", indent(level), s)
        }
    }
}

impl RenderableNode for AstIdentifier {
    fn render(&self, level: usize) -> String {
        match self {
            AstIdentifier::Variable(name) => format!("{}identifier: {} [variable]", indent(level), name),
            AstIdentifier::Let => format!("{}let [identifier]", indent(level))
        }
    }
}

impl RenderableNode for AstUnaryOperator {
    fn render(&self, level: usize) -> String {
        match self {
            AstUnaryOperator::Plus => format!("{}+ [unary operator]", indent(level)),
            AstUnaryOperator::Minus => format!("{}- [unary operator]", indent(level))
        }
    }
}

impl RenderableNode for AstBinaryOperator {
    fn render(&self, level: usize) -> String {
        match self {
            AstBinaryOperator::Add => format!("{}+ [binary operator]", indent(level)),
            AstBinaryOperator::Minus => format!("{}- [binary operator]", indent(level)),
            AstBinaryOperator::Multiply => format!("{}* [binary operator]", indent(level)),
            AstBinaryOperator::Divide => format!("{}/ [binary operator]", indent(level)),
            AstBinaryOperator::Equals => format!("{}= [binary operator]", indent(level))
        }
    }
}

impl RenderableNode for AstExpression {
    fn render(&self, level: usize) -> String {
        match self {
            AstExpression::Literal(lit, e_prime) =>
                format!("{}literal-expr\n{}\n{}", indent(level),
                        lit.render(level+1), (*e_prime).render(level+1)),

            AstExpression::Identifier(id, e_prime) =>
                format!("{}identifier-expr\n{}\n{}", indent(level),
                        id.render(level+1), (*e_prime).render(level+1)),

            AstExpression::Unary(unary, expr, e_prime) =>
                format!("{}unary-expr\n{}\n{}\n{}", indent(level),
                        unary.render(level+1), (*expr).render(level+1), (*e_prime).render(level+1))
        }
    }
}

impl RenderableNode for AstExpressionPrime {
    fn render(&self, level: usize) -> String {
        match self {
            AstExpressionPrime::Binary(binary, expr, e_prime) =>
                format!("{}binary-expr\n{}\n{}\n{}", indent(level),
                        binary.render(level+1), (*expr).render(level+1), (*e_prime).render(level+1)),

            AstExpressionPrime::EPS => format!("{}\u{03B5}", indent(level))
        }
    }
}

impl RenderableNode for AstStatement {
    fn render(&self, level: usize) -> String {
        match self {
            AstStatement::Assignment(id, expr) =>
                format!("{}let-assignment\n{}\n{}", indent(level),
                        id.render(level+1), expr.render(level+1))
        }
    }
}


fn indent(level: usize) -> String {
    str::repeat("    ", level)
}