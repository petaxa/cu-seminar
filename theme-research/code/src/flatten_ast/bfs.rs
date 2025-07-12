use super::Ast;
use std::collections::VecDeque;

const ROOT_NODE_INDEX: usize = 1;
const NODE_ELE_COUNT: usize = 4;

const CHILD_COORD: usize = 1;
const NEXT_COORD: usize = 2;

pub fn bfs(ast: &Ast, _log_queue: &mut Vec<String>) {
    let nodes = &ast.nodes;
    let mut q = VecDeque::new();

    let root_child = nodes[ROOT_NODE_INDEX * NODE_ELE_COUNT + CHILD_COORD];
    if root_child != 0 {
        q.push_back(root_child);
    }

    let root_next = nodes[ROOT_NODE_INDEX * NODE_ELE_COUNT + NEXT_COORD];
    if root_next != 0 {
        q.push_back(root_next);
    }

    while let Some(index) = q.pop_front() {
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
