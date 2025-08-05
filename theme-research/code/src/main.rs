mod flatten_ast;
mod pure_ast;
mod scenario;
mod utils;
use clap::Parser;
use scenario::count_malloc;
use scenario::elapsed;
use scenario::footprints;

#[derive(Parser)]
#[command(name = "bench_ast")]
#[command(about = "Benchmark pure-AST against flatten-AST")]
struct Cli {
    #[arg(short, long)]
    elapsed: bool,

    #[arg(short, long)]
    footprints: bool,

    #[arg(short, long)]
    malloc_count: bool,
}

fn main() {
    let source_text = "if(condition) { foo(); }";

    let cli = Cli::parse();
    let is_elasped = cli.elapsed;
    let is_footprints = cli.footprints;
    let is_malloc_count = cli.malloc_count;

    pure_bfs(source_text, is_elasped, is_footprints, is_malloc_count);
    flatten_bfs(source_text, is_elasped, is_footprints, is_malloc_count);
}

fn pure_bfs(
    source_text: &'static str,
    is_elasped: bool,
    is_footprints: bool,
    is_malloc_count: bool,
) {
    println!("BFS Pure AST");
    let asts: Vec<pure_ast::AstNode> = pure_ast::parse(source_text);

    if is_elasped {
        let elapsed = elapsed::bfs_pure_ast(asts.clone());
        println!("実行時間: {:?}", elapsed);
    }

    if is_footprints {
        let footprints = footprints::bfs_pure_ast(asts.clone());
        println!("Footprints: {:?}", footprints);
    }

    if is_malloc_count {
        count_malloc::bfs_pure_ast(asts.clone());
    }
}

fn flatten_bfs(
    source_text: &'static str,
    is_elasped: bool,
    is_footprints: bool,
    is_malloc_count: bool,
) {
    println!("BFS Flatten AST");
    let asts: flatten_ast::Ast = flatten_ast::parse(source_text);

    if is_elasped {
        let elapsed = elapsed::bfs_flatten_ast(asts.clone());
        println!("実行時間: {:?}", elapsed);
    }

    if is_footprints {
        let footprints = footprints::bfs_flatten_ast(asts.clone());
        println!("Footprints: {:?}", footprints);
    }

    if is_malloc_count {
        count_malloc::bfs_flatten_ast(asts.clone());
    }
}
