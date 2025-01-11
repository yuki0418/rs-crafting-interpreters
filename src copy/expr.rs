///
/// Literals: Numbers, strings, Booleans, and nil.
/// Unary expressions: A prefix ! to perform a logical not, and - to negate a number.
/// Binary expressions: The infix arithmetic (+, -, *, /) and logic operators (==, !=, <, <=, >, >=) we know and love.
/// Parentheses: A pair of ( and ) wrapped around an expression.
///
use crate::token::Token;

enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
    Nil,
}

struct Binary {
    left: Box<Expr>,
    operator: Box<Token>,
    right: Box<Expr>,
}

struct Grouping {
    expression: Box<Expr>,
}

struct Unary {
    operator: Box<Token>,
    right: Box<Expr>,
}

enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

// impl Expr {
//     fn accept(&self, visitor: &Visitor) -> &Self {
//         match self {
//             // Expr::Binary(binary) => binary.accept(visitor),
//             // Expr::Grouping(grouping) => grouping.accept(visitor),
//             Expr::Literal(literal) => visitor.visit_literal(visitor),
//             // Expr::Unary(unary) => unary.accept(visitor),
//         }
//     }
// }

// struct Visitor {}

// impl Visitor {
//     fn visit_binary<'a>(&self, binary: &'a Binary) -> &'a Binary {
//         binary
//     }
//     fn visit_grouping<'a>(&self, grouping: &'a Grouping) -> &'a Grouping {
//         grouping
//     }
//     fn visit_literal<'a>(&self, literal: &'a Literal) -> &'a Literal {
//         literal
//     }
//     fn visit_unary<'a>(&self, unary: &'a Unary) -> &'a Unary {
//         unary
//     }
// }

// trait AcceptVisitorTrait {
//     fn accept(&self, visitor: &Visitor) -> &Self;
// }

// impl AcceptVisitorTrait for Binary {
//     fn accept(&self, visitor: &Visitor) -> &Self {
//         visitor.visit_binary(self)
//     }
// }

// impl AcceptVisitorTrait for Grouping {
//     fn accept(&self, visitor: &Visitor) -> &Self {
//         visitor.visit_grouping(self)
//     }
// }

// impl AcceptVisitorTrait for Literal {
//     fn accept(&self, visitor: &Visitor) -> &Self {
//         visitor.visit_literal(self)
//     }
// }

// impl AcceptVisitorTrait for Unary {
//     fn accept(&self, visitor: &Visitor) -> &Self {
//         visitor.visit_unary(self)
//     }
// }
