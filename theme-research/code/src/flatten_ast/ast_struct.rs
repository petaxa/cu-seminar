use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Ast {
    pub string_table: Vec<String>,
    pub properties: Vec<AstProps>,
    pub nodes: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum AstProps {
    IfStatement {
        test: usize,
        consequent: usize,
        alternate: usize,
    },
    BlockStatement {
        stmts: Vec<usize>,
    },
    ExpressionStatement {
        expression: usize,
    },
    CallExpression {
        callee: usize,
        arguments: Vec<usize>,
        type_arguments: usize,
    },
    Identifier {
        value: String,
        optional: bool,
    },
    Empty {},
}
