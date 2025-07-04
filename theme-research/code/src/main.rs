mod flatten_ast;
mod pure_ast;
mod utils;

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

    pure_ast::bfs(pure_ast::QueueItem::Multiple(asts));
}

fn flatten_bfs(source_text: &'static str) {
    println!("{}", "BFS Flatten AST");
    let asts: Vec<flatten_ast::Ast> = flatten_ast::parse(source_text);

    let json_asts = serde_json::to_string_pretty(&asts[0]).unwrap();
    println!("{}", json_asts);

    flatten_ast::bfs(asts);
}
