mod bfs;
mod parse;
use crate::utils::parse_to_deno_ast;
use bfs::process_node;
use parse::parse_to_pure_ast;
use serde::Deserialize;
use std::collections::VecDeque;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "type")]
pub enum AstNode {
    IfStatement {
        span: Span,
        test: Box<AstNode>,
        consequent: Box<AstNode>,
        alternate: Option<Box<AstNode>>,
    },
    BlockStatement {
        span: Span,
        ctxt: u32,
        stmts: Vec<AstNode>,
    },
    ExpressionStatement {
        span: Span,
        expression: Box<AstNode>,
    },
    CallExpression {
        span: Span,
        ctxt: u32,
        callee: Box<AstNode>,
        arguments: Vec<AstNode>,
        type_arguments: Option<()>,
    },
    Identifier {
        span: Span,
        ctxt: u32,
        value: String,
        optional: bool,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Span {
    start: u32,
    end: u32,
}

pub fn parse(source_text: &'static str) -> Vec<AstNode> {
    let parsed_source = parse_to_deno_ast(source_text);
    let asts = parse_to_pure_ast(parsed_source);

    return asts;
}

pub enum QueueItem {
    Single(Box<AstNode>),
    Multiple(Vec<AstNode>),
}

pub fn bfs(root: QueueItem) {
    let mut queue: VecDeque<QueueItem> = VecDeque::new();
    queue.push_back(root);

    while let Some(item) = queue.pop_front() {
        match item {
            QueueItem::Single(node) => {
                process_node(*node, &mut queue);
            }
            QueueItem::Multiple(nodes) => {
                for (index, node) in nodes.into_iter().enumerate() {
                    println!("touch node[{}]", index);
                    process_node(node, &mut queue);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_ast_base() {
        let source_text = "if(condition) { foo(); }";
        let asts: Vec<AstNode> = parse(source_text);

        // タッチ順を示す
        // cargo test bfs_ast_base -- --nocapture を実行して目視で確認(TODO: 自動化)
        // レベル 0 (ルート)
        // - root[0]

        // レベル 1 (IfStatement の直接の子)
        // - root[0].test
        // - root[0].consequent
        // - root[0].alternate

        // レベル 2 (レベル1の子ノード)
        // - root[0].test.ctxt
        // - root[0].test.value
        // - root[0].test.optional
        // - root[0].consequent.ctxt
        // - root[0].consequent.stmts

        // レベル 3 (レベル2の子ノード)
        // - root[0].consequent.stmts[0]

        // レベル 4 (レベル3の子ノード)
        // - root[0].consequent.stmts[0].expression

        // レベル 5 (レベル4の子ノード)
        // - root[0].consequent.stmts[0].expression.ctxt
        // - root[0].consequent.stmts[0].expression.callee
        // - root[0].consequent.stmts[0].expression.arguments
        // - root[0].consequent.stmts[0].expression.type_arguments

        // レベル 6 (レベル5の子ノード)
        // - root[0].consequent.stmts[0].expression.callee.ctxt
        // - root[0].consequent.stmts[0].expression.callee.value
        // - root[0].consequent.stmts[0].expression.callee.optional
        bfs(QueueItem::Multiple(asts));
    }

    #[test]
    fn parse_to_ast_test() {
        let source_text = "if(condition) { foo(); }";
        let asts = parse(source_text);
        let expected = AstNode::IfStatement {
            span: Span { start: 1, end: 25 },
            test: Box::new(AstNode::Identifier {
                span: Span { start: 4, end: 13 },
                ctxt: 0,
                value: "condition".to_string(),
                optional: false,
            }),
            consequent: Box::new(AstNode::BlockStatement {
                span: Span { start: 15, end: 25 },
                ctxt: 0,
                stmts: vec![AstNode::ExpressionStatement {
                    span: Span { start: 17, end: 23 },
                    expression: Box::new(AstNode::CallExpression {
                        span: Span { start: 17, end: 22 },
                        ctxt: 0,
                        callee: Box::new(AstNode::Identifier {
                            span: Span { start: 17, end: 20 },
                            ctxt: 0,
                            value: "foo".to_string(),
                            optional: false,
                        }),
                        arguments: vec![],
                        type_arguments: None,
                    }),
                }],
            }),
            alternate: None,
        };

        println!("{:?}", &asts);
        assert_eq!(vec![expected], asts);
    }

    #[test]
    fn parse_to_some_ast() {
        {
            let source_text = "if(condition) { foo(); }";
            let ast = parse(source_text);

            println!("{:?}", ast)
        }
    }
}
