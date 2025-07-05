mod flatten_ast;
mod pure_ast;
mod utils;
use std::time::Instant;

fn main() {
    let source_text = "if(condition) { foo(); }";

    pure_bfs(source_text);

    flatten_bfs(source_text);
}

fn pure_bfs(source_text: &'static str) {
    println!("{}", "BFS Pure AST");
    let asts: Vec<pure_ast::AstNode> = pure_ast::parse(source_text);

    let json_asts = serde_json::to_string_pretty(&asts[0]).unwrap();
    println!("{}", json_asts);

    let pure_fnc_time = Instant::now();
    pure_ast::bfs(pure_ast::QueueItem::Multiple(asts));
    println!("Pure AST 実行時間: {:?}", pure_fnc_time.elapsed());
}

fn flatten_bfs(source_text: &'static str) {
    println!("{}", "BFS Flatten AST");
    let asts: Vec<flatten_ast::Ast> = flatten_ast::parse(source_text);

    let json_asts = serde_json::to_string_pretty(&asts[0]).unwrap();
    println!("{}", json_asts);

    let flatten_fnc_time = Instant::now();
    flatten_ast::bfs(asts);
    println!("Flatten AST 実行時間: {:?}", flatten_fnc_time.elapsed());
}
