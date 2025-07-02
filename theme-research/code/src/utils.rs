use deno_ast::{parse_script, MediaType, ParseParams, ParsedSource, SourceTextInfo};

pub(crate) fn parse_to_deno_ast(source_text: &'static str) -> ParsedSource {
    // deno_lint で AST にパース
    let text_info = SourceTextInfo::new(source_text.into());
    let parsed_source = parse_script(ParseParams {
        specifier: deno_ast::ModuleSpecifier::parse("file:///my_file.ts").unwrap(),
        media_type: MediaType::TypeScript,
        text: text_info.text(),
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: false,
    })
    .expect("should parse");

    return parsed_source;
}
