use super::ast_struct::AstNode;
use crate::utils::parse_to_deno_ast;
use deno_ast::ParsedSource;

pub fn parse(source_text: &'static str) -> Vec<AstNode> {
    let parsed_source = parse_to_deno_ast(source_text);
    let asts = parse_to_pure_ast(parsed_source);

    return asts;
}

fn parse_to_pure_ast(parsed_source: ParsedSource) -> Vec<AstNode> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_to_ast_test() {
        let source_text = "if(condition) { foo(); }";
        let asts = parse(source_text);
        let expected = AstNode::IfStatement {
            test: Box::new(AstNode::Identifier {
                value: "condition".to_string(),
                optional: false,
            }),
            consequent: Box::new(AstNode::BlockStatement {
                stmts: vec![AstNode::ExpressionStatement {
                    expression: Box::new(AstNode::CallExpression {
                        callee: Box::new(AstNode::Identifier {
                            value: "foo".to_string(),
                            optional: false,
                        }),
                        arguments: vec![],
                        type_arguments: None,
                    }),
                }],
            }),
            alternate: None,
        };

        println!("{:?}", &asts);
        assert_eq!(vec![expected], asts);
    }

    #[test]
    fn parse_to_some_ast() {
        {
            let source_text = "if(condition) { foo(); }";
            let ast = parse(source_text);

            println!("{:?}", ast)
        }
    }
}
