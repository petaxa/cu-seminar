use super::Ast;
use std::collections::VecDeque;

const ROOT_NODE_INDEX: usize = 1;
const NODE_ELE_COUNT: usize = 4;

const CHILD_COORD: usize = 1;
const NEXT_COORD: usize = 2;

pub fn bfs(ast: Vec<Ast>, log_queue: &mut Vec<String>) {
    for (index, node) in ast.into_iter().enumerate() {
        log_queue.push(format!("touch node[{}]", index));
        process_node(node, log_queue);
    }
}

fn process_node(ast: Ast, log_queue: &mut Vec<String>) {
    // nodes をたどる → type の String を見る → 同 index の prop を見る → props の中身を一個ずつキューに入れる → キューの先頭から処理
    let nodes = ast.nodes;
    let props = ast.properties;

    let mut q: VecDeque<usize> = VecDeque::new();

    log_queue.push(format!("{:?}", props[ROOT_NODE_INDEX].clone()));
    let root_child = nodes[ROOT_NODE_INDEX * NODE_ELE_COUNT + CHILD_COORD];
    if root_child != 0 {
        q.push_back(root_child);
    }
    let root_next = nodes[ROOT_NODE_INDEX * NODE_ELE_COUNT + NEXT_COORD];
    if root_next != 0 {
        q.push_back(root_next);
    }

    while let Some(index) = q.pop_front() {
        log_queue.push(format!("{:?}", props[index].clone()));

        let base = index * NODE_ELE_COUNT;
        let child = nodes[base + CHILD_COORD];
        if child != 0 {
            q.push_back(child);
        }

        let next = nodes[base + NEXT_COORD];
        if next != 0 {
            q.push_back(next);
        }
    }
}
