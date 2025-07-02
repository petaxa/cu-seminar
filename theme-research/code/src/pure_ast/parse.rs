use super::AstNode;
use deno_ast::ParsedSource;

pub(super) fn parse_to_pure_ast(parsed_source: ParsedSource) -> Vec<AstNode> {
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
