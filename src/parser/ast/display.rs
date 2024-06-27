use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use crate::parser::ast::{Expr, Stmt, Type};
use crate::parser::utilities::ToDoc;

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
                explicit_type,
            } => {
                let mut docs = vec![
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
                ];
                docs.push(allocator.text("Value: ").nest(2));
                docs.push(
                    if let Some(assigned_value) = assigned_value {
                        assigned_value.to_doc(&allocator)
                    } else {
                        allocator.text("None")
                    }
                        .nest(2)
                        .append(allocator.line()),
                );
                docs.push(allocator.text("Type: ").nest(2));
                docs.push(
                    if let Some(explicit_type) = explicit_type {
                        explicit_type.to_doc(&allocator)
                    } else {
                        allocator.text("None")
                    }
                        .nest(2)
                        .append(allocator.line()),
                );

                allocator.concat(docs)
            }
        }
    }
}

impl ToDoc for Type {
    fn to_doc<'a>(&self, allocator: &'a BoxAllocator) -> DocBuilder<'a, BoxAllocator> {
        match self {
            Type::Symbol(name) => allocator.text(name.clone()),
            Type::Array(subtype) => allocator.text("Array<").append(subtype.to_doc(&allocator).nest(2)).append(allocator.text(">")),
        }
    }
}
