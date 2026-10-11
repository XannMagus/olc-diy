use xann_core::OperatorKind;

pub use structs::StructFieldInfo;

mod display;
mod structs;

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
        assigned_value: Option<Expr>,
        explicit_type: Option<Type>,
    },
    StructDeclaration {
        identifier: String,
        fields: Vec<StructFieldInfo>,
    },
}

#[derive(Debug)]
pub enum Type {
    Symbol(String),
    Array(Box<Type>),
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
        Self::Prefix {
            operator,
            right: Box::new(right),
        }
    }
}

impl Stmt {
    pub fn variable_declaration(
        identifier: String,
        constant: bool,
        assigned_value: Option<Expr>,
        explicit_type: Option<Type>,
    ) -> Self {
        Self::VariableDeclaration {
            identifier,
            constant,
            assigned_value,
            explicit_type,
        }
    }
}
