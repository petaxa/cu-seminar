use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "type")]
pub enum AstNode {
    IfStatement {
        test: Box<AstNode>,
        consequent: Box<AstNode>,
        alternate: Option<Box<AstNode>>,
    },
    BlockStatement {
        stmts: Vec<AstNode>,
    },
    ExpressionStatement {
        expression: Box<AstNode>,
    },
    CallExpression {
        callee: Box<AstNode>,
        arguments: Vec<AstNode>,
        type_arguments: Option<()>,
    },
    Identifier {
        value: String,
        optional: bool,
    },
}
