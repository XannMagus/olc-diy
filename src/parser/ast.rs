use std::fmt::Display;

use pretty::{BoxAllocator, DocAllocator, DocBuilder};

use crate::lexer::OperatorKind;
use crate::parser::utilities::ToDoc;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    String(String),
    Symbol(String),
    Binary {
        left: Box<Expr>,
        operator: OperatorKind,
        right: Box<Expr>,
    },
    Prefix {
        operator: OperatorKind,
        right: Box<Expr>,
    },
    Assignment {
        assignee: Box<Expr>,
        operator: OperatorKind,
        value: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    VariableDeclaration {
        identifier: String,
        constant: bool,
        assigned_value: Expr,
    },
}

impl Expr {
    pub fn binary(left: Expr, operator: OperatorKind, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn assignment(assignee: Expr, operator: OperatorKind, value: Expr) -> Self {
        Self::Assignment {
            assignee: Box::new(assignee),
            operator,
            value: Box::new(value),
        }
    }

    pub fn prefix(operator: OperatorKind, right: Expr) -> Self {
        Self::Prefix { operator, right: Box::new(right) }
    }
}

impl Stmt {
    pub fn variable_declaration(identifier: String, constant: bool, assigned_value: Expr) -> Self {
        Self::VariableDeclaration {
            identifier,
            constant,
            assigned_value,
        }
    }
}

impl ToDoc for Expr {
    fn to_doc<'a>(&self, allocator: &'a BoxAllocator) -> DocBuilder<'a, BoxAllocator> {
        match self {
            Expr::Number(n) => allocator.text(n.to_string()),
            Expr::String(s) => allocator.text(format!("\"{s}\"")),
            Expr::Symbol(name) => allocator.text(name.clone()),
            Expr::Binary {
                left,
                operator,
                right,
            } => allocator.concat([
                allocator.line().append(allocator.text("left: ")),
                left.to_doc(allocator).nest(2),
                allocator.line(),
                allocator
                    .text("operator: ")
                    .append(allocator.text(operator.to_string())),
                allocator.line(),
                allocator.text("right: "),
                right.to_doc(allocator).nest(2),
            ]),
            Expr::Assignment {
                assignee,
                operator,
                value,
            } => allocator.concat([
                allocator.line().append(allocator.text("assignee: ")),
                assignee.to_doc(allocator).nest(2),
                allocator.line(),
                allocator
                    .text("operator: ")
                    .append(allocator.text(operator.to_string())),
                allocator.line(),
                allocator.text("value: "),
                value.to_doc(allocator).nest(2),
            ]),
            Expr::Prefix { operator, right } => allocator.concat([
                allocator.line().append(allocator.text("operator: ")),
                allocator.text(operator.to_string()),
                allocator.line(),
                allocator.text("right: "),
                right.to_doc(allocator).nest(2),
            ]),
        }
    }
}

impl ToDoc for Stmt {
    fn to_doc<'a>(&self, allocator: &'a BoxAllocator) -> DocBuilder<'a, BoxAllocator> {
        match self {
            Stmt::Block(stmts) => {
                let mut docs = vec![allocator.text("{"), allocator.line()];
                for stmt in stmts {
                    docs.push(stmt.to_doc(&allocator).nest(2).append(allocator.line()));
                }
                docs.push(allocator.text("}"));
                allocator.concat(docs)
            }
            Stmt::Expression(expr) => allocator.concat([
                allocator.text("Expression: "),
                expr.to_doc(&allocator).nest(2),
            ]),
            Stmt::VariableDeclaration {
                identifier,
                constant,
                assigned_value,
            } => {
                allocator.concat([
                    allocator
                        .text("Variable Declaration")
                        .append(allocator.line()),
                    allocator.text("Identifier: ").nest(2),
                    allocator
                        .text(identifier.clone())
                        .nest(2)
                        .append(allocator.line()),
                    allocator.text("Constant? ").nest(2),
                    allocator
                        .text(constant.to_string())
                        .nest(2)
                        .append(allocator.line()),
                    allocator.text("Value: ").nest(2),
                    assigned_value.to_doc(&allocator).nest(2).append(allocator.line()),
                ])
            }
        }
    }
}