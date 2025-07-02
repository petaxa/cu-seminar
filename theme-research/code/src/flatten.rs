/// 全く動かない仮実装
///

#[derive(Debug, Serialize, PartialEq)]
pub struct FlatAst {
    pub string_table: Vec<String>,
    pub properties: Vec<FlatProps>,
    pub nodes: Vec<usize>,
}

#[derive(Debug, Serialize, Default, PartialEq)]
pub struct FlatProps {
    pub test: Option<usize>,
    pub consequent: Option<usize>,
    pub alternate: Option<usize>,
    pub name: Option<String>,
    pub expression: Option<usize>,
    pub body: Option<Vec<usize>>,
}

struct FlattenContext {
    string_table: Vec<String>,
    string_map: HashMap<String, usize>,
    properties: Vec<FlatProps>,
    nodes: Vec<usize>,
    node_counter: usize,
}

impl FlattenContext {
    fn new() -> Self {
        let mut ctx = FlattenContext {
            string_table: Vec::new(),
            string_map: HashMap::new(),
            properties: Vec::new(),
            nodes: Vec::new(),
            node_counter: 0,
        };

        // Add empty at index 0
        ctx.add_string("".to_string());
        ctx.nodes.extend_from_slice(&[0, 0, 0, 0]);
        ctx.properties.push(FlatProps::default());
        ctx
    }

    fn add_string(&mut self, s: String) -> usize {
        if let Some(&index) = self.string_map.get(&s) {
            index
        } else {
            let index = self.string_table.len();
            self.string_table.push(s.clone());
            self.string_map.insert(s, index);
            index
        }
    }

    fn get_node_type_index(&mut self, node: &AstNode) -> usize {
        let type_name = match node {
            AstNode::IfStatement { .. } => "IfStatement",
            AstNode::BlockStatement { .. } => "BlockStatement",
            AstNode::ExpressionStatement { .. } => "ExpressionStatement",
            AstNode::CallExpression { .. } => "CallExpression",
            AstNode::Identifier { .. } => "Identifier",
        };
        self.add_string(type_name.to_string())
    }
}

fn flatten_ast(ast: &AstNode) -> FlatAst {
    let mut ctx = FlattenContext::new();

    let _root_index = flatten_node(ast, &mut ctx, 0);

    FlatAst {
        string_table: ctx.string_table,
        properties: ctx.properties,
        nodes: ctx.nodes,
    }
}

fn flatten_node(node: &AstNode, ctx: &mut FlattenContext, parent_index: usize) -> usize {
    let current_index = ctx.node_counter;
    ctx.node_counter += 1;

    let type_index = ctx.get_node_type_index(node);
    let props_index = ctx.properties.len();

    // Reserve space in nodes array (4 elements per node)
    let node_start = ctx.nodes.len();
    ctx.nodes.extend_from_slice(&[0, 0, 0, 0]);

    let mut props = FlatProps::default();

    match node {
        AstNode::IfStatement {
            test,
            consequent,
            alternate,
            ..
        } => {
            let test_index = flatten_node(test, ctx, current_index);
            let consequent_index = flatten_node(consequent, ctx, current_index);

            props.test = Some(test_index);
            props.consequent = Some(consequent_index);
            props.alternate = Some(consequent_index);

            if let Some(alternate_node) = alternate {
                let alternate_index = flatten_node(alternate_node, ctx, current_index);
                props.alternate = Some(alternate_index);
            } else {
                props.alternate = Some(0)
            }
        }

        AstNode::BlockStatement { stmts, .. } => {
            let mut stmt_indices = Vec::new();
            for stmt in stmts {
                let stmt_index = flatten_node(stmt, ctx, current_index);
                stmt_indices.push(stmt_index);
            }
            if !stmt_indices.is_empty() {
                props.body = Some(stmt_indices);
            }
        }

        AstNode::ExpressionStatement { expression, .. } => {
            let expr_index = flatten_node(expression, ctx, current_index);
            props.expression = Some(expr_index);
        }

        AstNode::CallExpression {
            callee, arguments, ..
        } => {
            let callee_index = flatten_node(callee, ctx, current_index);
            // For simplicity, we're treating callee as expression
            // In a more complete implementation, you'd want separate fields
            props.expression = Some(callee_index);

            // Handle arguments if needed
            if !arguments.is_empty() {
                let mut arg_indices = Vec::new();
                for arg in arguments {
                    let arg_index = flatten_node(arg, ctx, current_index);
                    arg_indices.push(arg_index);
                }
                // You might want to add an arguments field to FlatProps
            }
        }

        AstNode::Identifier { value, .. } => {
            props.name = Some(value.clone());
        }
    }
    ctx.properties.push(props);

    ctx.nodes[node_start] = type_index;
    ctx.nodes[node_start + 1] = props_index;
    ctx.nodes[node_start + 2] = parent_index;
    ctx.nodes[node_start + 3] = current_index;

    current_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_ast_base() {
        let ast = AstNode::IfStatement {
            span: Span { start: 1, end: 25 },
            test: Box::new(AstNode::Identifier {
                span: Span { start: 4, end: 13 },
                ctxt: 0,
                value: "condition".to_string(),
                optional: false,
            }),
            consequent: Box::new(AstNode::BlockStatement {
                span: Span { start: 15, end: 25 },
                ctxt: 0,
                stmts: vec![AstNode::ExpressionStatement {
                    span: Span { start: 17, end: 23 },
                    expression: Box::new(AstNode::CallExpression {
                        span: Span { start: 17, end: 22 },
                        ctxt: 0,
                        callee: Box::new(AstNode::Identifier {
                            span: Span { start: 17, end: 20 },
                            ctxt: 0,
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
        let expected = FlatAst {
            string_table: vec![
                "".to_string(),
                "IfStatement".to_string(),
                "Identifier".to_string(),
                "BlockStatement".to_string(),
                "ExpressionStatement".to_string(),
                "CallExpression".to_string(),
            ],
            properties: vec![
                FlatProps::default(),
                FlatProps {
                    test: Some(2),
                    consequent: Some(3),
                    alternate: Some(0),
                    ..Default::default()
                },
                FlatProps {
                    name: Some("condition".to_string()),
                    ..Default::default()
                },
                FlatProps {
                    body: Some(vec![4]),
                    ..Default::default()
                },
                FlatProps {
                    expression: Some(5),
                    ..Default::default()
                },
            ],
            nodes: vec![
                0, 0, 0, 0, 1, 2, 0, 0, 2, 0, 3, 1, 3, 4, 0, 1, 4, 5, 0, 3, 5, 0, 0, 4,
            ],
        };

        let flatten = flatten_ast(&ast);
        assert_eq!(expected, flatten);
    }
}
