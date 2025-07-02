mod pure_ast;
mod utils;

fn main() {
    let source_text = "if(condition) { foo(); }";
    let asts: Vec<pure_ast::AstNode> = pure_ast::parse(source_text);

    pure_ast::bfs(pure_ast::QueueItem::Multiple(asts));
}
