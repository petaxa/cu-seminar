use crate::{flatten_ast, pure_ast};

pub fn bfs_pure_ast(asts: Vec<pure_ast::AstNode>) -> Vec<String> {
    let log_queue: &mut Vec<String> = &mut vec![];
    pure_ast::bfs(pure_ast::QueueItem::Multiple(asts), log_queue);

    return log_queue.to_vec();
}

