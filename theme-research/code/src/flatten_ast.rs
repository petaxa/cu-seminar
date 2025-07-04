mod bfs;
mod parse;
use crate::pure_ast;
use bfs::process_node;
use parse::parse_to_flatten_ast;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
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

pub fn parse(source_text: &'static str) -> Vec<Ast> {
    let pure_ast = pure_ast::parse(source_text);
    let asts = parse_to_flatten_ast(pure_ast);

    return asts;
}

pub fn bfs(root: Vec<Ast>) {
    for (index, node) in root.into_iter().enumerate() {
        println!("touch node[{}]", index);
        process_node(node);
    }
}
