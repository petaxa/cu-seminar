use super::{AstNode, QueueItem};
use std::collections::VecDeque;

pub(super) fn process_node(node: AstNode, queue: &mut VecDeque<QueueItem>) {
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
