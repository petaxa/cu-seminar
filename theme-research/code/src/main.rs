use serde::Deserialize;
use std::collections::VecDeque;

use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceTextInfo;
use deno_ast::parse_script;

fn main() {
    let source_text = "if(condition) { foo(); }";
    let asts: Vec<AstNode> = parse_to_ast(source_text);

    // ===BFSを実装===
    bfs_ast(QueueItem::Multiple(asts));
}

fn parse_to_ast(source_text: &'static str) -> Vec<AstNode> {
    // deno_lint で AST にパース
    let text_info = SourceTextInfo::new(source_text.into());
    let parsed_source = parse_script(ParseParams {
        specifier: deno_ast::ModuleSpecifier::parse("file:///my_file.ts").unwrap(),
        media_type: MediaType::TypeScript,
        text: text_info.text(),
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: false,
    })
    .expect("should parse");

    // JSON を介して 自作の AST Struct にパース
    let program_ref = parsed_source.program_ref();
    let bodies = &program_ref.unwrap_script().body;
    let asts: Vec<AstNode> = bodies
        .iter()
        .map(|body| {
            let json = serde_json::to_string_pretty(body).unwrap();
            serde_json::from_str(&json).unwrap()
        })
        .collect();

    return asts;
}

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

enum QueueItem {
    Single(Box<AstNode>),
    Multiple(Vec<AstNode>),
}

fn process_node(node: AstNode, queue: &mut VecDeque<QueueItem>) {
    match node {
        AstNode::IfStatement {
            test,
            consequent,
            alternate,
            ..
        } => {
            println!("touch test");
            queue.push_back(QueueItem::Single(test));

            println!("touch consequent");
            queue.push_back(QueueItem::Single(consequent));

            println!("touch alternate");
            if let Some(alt) = alternate {
                queue.push_back(QueueItem::Single(alt));
            }
        }

        AstNode::BlockStatement { ctxt, stmts, .. } => {
            println!("touch ctxt: {}", ctxt);

            println!("touch stmts");
            queue.push_back(QueueItem::Multiple(stmts));
        }

        AstNode::ExpressionStatement { expression, .. } => {
            println!("touch expression");
            queue.push_back(QueueItem::Single(expression));
        }

        AstNode::CallExpression {
            callee,
            arguments,
            ctxt,
            type_arguments,
            ..
        } => {
            println!("touch ctxt: {}", ctxt);

            println!("touch callee");
            queue.push_back(QueueItem::Single(callee));

            println!("touch arguments");
            queue.push_back(QueueItem::Multiple(arguments));

            println!("touch type_arguments: {:?}", type_arguments);
        }

        AstNode::Identifier {
            ctxt,
            value,
            optional,
            ..
        } => {
            println!("touch ctxt: {}", ctxt);
            println!("touch value: {}", value);
            println!("touch optional: {}", optional);
        }
    }
}

fn bfs_ast(root: QueueItem) {
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
    fn parse_to_ast_test() {
        let source_text = "if(condition) { foo(); }";
        let asts = parse_to_ast(source_text);
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
            let ast = parse_to_ast(source_text);

            println!("{:?}", ast)
        }
    }

    #[test]
    fn bfs_ast_base() {
        let source_text = "if(condition) { foo(); }";
        let asts: Vec<AstNode> = parse_to_ast(source_text);

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
        bfs_ast(QueueItem::Multiple(asts));
    }
}
