use super::ast_struct::AstNode;
use std::collections::VecDeque;

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

        AstNode::BlockStatement { stmts, .. } => {
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
            type_arguments,
            ..
        } => {
            println!("touch callee");
            queue.push_back(QueueItem::Single(callee));

            println!("touch arguments");
            queue.push_back(QueueItem::Multiple(arguments));

            println!("touch type_arguments: {:?}", type_arguments);
        }

        AstNode::Identifier {
            value, optional, ..
        } => {
            println!("touch value: {}", value);
            println!("touch optional: {}", optional);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse::parse;
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
}
