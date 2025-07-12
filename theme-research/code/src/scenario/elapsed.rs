use crate::{flatten_ast, pure_ast};
use std::time::{Duration, Instant};

pub fn bfs_pure_ast(asts: &Vec<pure_ast::AstNode>) -> Duration {
    let log_queue: &mut Vec<String> = &mut vec![];
    let pure_fnc_time = Instant::now();
    pure_ast::bfs(pure_ast::QueueItem::Multiple(asts), log_queue);
    let elapsed = pure_fnc_time.elapsed();

    return elapsed;
}

pub fn bfs_flatten_ast(ast: Vec<flatten_ast::Ast>) -> Duration {
    let pure_fnc_time = Instant::now();
    flatten_ast::bfs(ast);
    let elapsed = pure_fnc_time.elapsed();

    return elapsed;
}
