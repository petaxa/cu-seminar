use super::ast_struct::AstNode;
use std::collections::VecDeque;

pub enum QueueItem<'a> {
    Single(&'a AstNode),
    Multiple(&'a Vec<AstNode>),
}

pub fn bfs(root: QueueItem, log_queue: &mut Vec<String>) {
    let mut queue: VecDeque<QueueItem> = VecDeque::new();
    queue.push_back(root);

    while let Some(item) = queue.pop_front() {
        match item {
            QueueItem::Single(node) => {
                process_node(node, &mut queue, log_queue);
            }
            QueueItem::Multiple(nodes) => {
                for (index, node) in nodes.into_iter().enumerate() {
                    log_queue.push(format!("touch node[{}]", index));
                    process_node(node, &mut queue, log_queue);
                }
            }
        }
    }
}

fn process_node<'a>(node: &'a AstNode, queue: &mut VecDeque<QueueItem<'a>>, log_queue: &mut Vec<String>) {
    match node {
        AstNode::IfStatement {
            test,
            consequent,
            alternate,
            ..
        } => {
            log_queue.push("touch test".to_string());
            queue.push_back(QueueItem::Single(test));

            log_queue.push("touch consequent".to_string());
            queue.push_back(QueueItem::Single(consequent));

            log_queue.push("touch alternate".to_string());
            if let Some(alt) = alternate {
                queue.push_back(QueueItem::Single(alt));
            }
        }

        AstNode::BlockStatement { stmts, .. } => {
            log_queue.push("touch stmts".to_string());
            queue.push_back(QueueItem::Multiple(stmts));
        }

        AstNode::ExpressionStatement { expression, .. } => {
            log_queue.push("touch expression".to_string());
            queue.push_back(QueueItem::Single(expression));
        }

        AstNode::CallExpression {
            callee,
            arguments,
            type_arguments,
            ..
        } => {
            log_queue.push("touch callee".to_string());
            queue.push_back(QueueItem::Single(callee));

            log_queue.push("touch arguments".to_string());
            queue.push_back(QueueItem::Multiple(arguments));

            log_queue.push(format!("touch type_arguments: {:?}", type_arguments));
        }

        AstNode::Identifier {
            value, optional, ..
        } => {
            log_queue.push(format!("touch value: {}", value));
            log_queue.push(format!("touch optional: {}", optional));
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
        let mut log_queue = Vec::new();
        bfs(QueueItem::Multiple(&asts), &mut log_queue);

        let expected = vec![
            "touch node[0]",
            "touch test",
            "touch consequent",
            "touch alternate",
            "touch value: condition",
            "touch optional: false",
            "touch stmts",
            "touch node[0]",
            "touch expression",
            "touch callee",
            "touch arguments",
            "touch type_arguments: None",
            "touch value: foo",
            "touch optional: false",
        ];
        assert_eq!(expected, log_queue);
    }
}
