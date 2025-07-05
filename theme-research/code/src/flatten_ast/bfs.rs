use super::Ast;
use std::collections::VecDeque;

const ROOT_NODE_INDEX: usize = 1;
const NODE_ELE_COUNT: usize = 4;

const CHILD_COORD: usize = 1;
const NEXT_COORD: usize = 2;

pub fn bfs(root: Vec<Ast>) {
    for (index, node) in root.into_iter().enumerate() {
        println!("touch node[{}]", index);
        process_node(node);
    }
}

fn process_node(ast: Ast) {
    // nodes をたどる → type の String を見る → 同 index の prop を見る → props の中身を一個ずつキューに入れる → キューの先頭から処理
    let nodes = ast.nodes;
    let props = ast.properties;

    let mut q: VecDeque<usize> = VecDeque::new();

    println!("{:?}", props[ROOT_NODE_INDEX].clone());
    let root_child = nodes[ROOT_NODE_INDEX * NODE_ELE_COUNT + CHILD_COORD];
    if root_child != 0 {
        q.push_back(root_child);
    }
    let root_next = nodes[ROOT_NODE_INDEX * NODE_ELE_COUNT + NEXT_COORD];
    if root_next != 0 {
        q.push_back(root_next);
    }

    while let Some(index) = q.pop_front() {
        println!("{:?}", props[index].clone());

        let child = nodes[index * NODE_ELE_COUNT + CHILD_COORD];
        if child != 0 {
            q.push_back(child);
        }

        let next = nodes[index * NODE_ELE_COUNT + NEXT_COORD];
        if next != 0 {
            q.push_back(next);
        }
    }
}
