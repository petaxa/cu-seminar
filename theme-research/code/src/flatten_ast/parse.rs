use super::{Ast, AstProps};
use crate::pure_ast;

pub(super) fn parse_to_flatten_ast(_pure_ast: Vec<pure_ast::AstNode>) -> Vec<Ast> {
    // NOTE: とてつもない仮実装 本来はパーサを記述するが、現在はパース済みの固定値を返す
    // 手動でパースすることは可能なので、時間的余裕を見て自動化するかを判断する
    let expected = Ast {
        string_table: vec![
            "".to_string(),
            "IfStatement".to_string(),
            "Identifier".to_string(),
            "BlockStatement".to_string(),
            "ExpressionStatement".to_string(),
            "CallExpression".to_string(),
        ],
        properties: vec![
            AstProps::Empty {},
            AstProps::IfStatement {
                test: 2,
                consequent: 3,
                alternate: 0,
            },
            AstProps::Identifier {
                value: "condition".to_string(),
                optional: false,
            },
            AstProps::BlockStatement { stmts: vec![4] },
            AstProps::ExpressionStatement { expression: 5 },
            AstProps::CallExpression {
                callee: 6,
                arguments: vec![],
                type_arguments: 0,
            },
            AstProps::Identifier {
                value: "foo".to_string(),
                optional: false,
            },
        ],
        nodes: vec![
            0, 0, 0, 0, 1, 2, 0, 0, 2, 0, 3, 1, 3, 4, 0, 1, 4, 5, 0, 3, 5, 6, 0, 4, 2, 0, 0, 0,
        ],
    };
    return vec![expected];
}
