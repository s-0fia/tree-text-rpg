#![allow(dead_code)]
use std::{collections::HashMap, rc::Rc};

pub struct Node {
    pub next_nodes: Vec<RcNode>,
    pub contents: String,
}

pub type RcNode = Rc<Node>;

pub struct Graph {
    pub head: RcNode,
    pub curr: RcNode,
}

pub type IndexedNodeList = Vec<(String, usize, Vec<usize>)>;

impl Graph {
    pub fn parse(node_list: IndexedNodeList) -> Option<Self> {
        let mut nodes: HashMap<usize, RcNode> = HashMap::new();

        for (contents, idx, _) in node_list.iter() {
            let new_node = Rc::new(Node {
                next_nodes: vec![],
                contents: contents.to_owned(),
            });

            nodes.insert(*idx, new_node);
        }

        if nodes.get(&0).is_none() {
            return None;
        }

        for (_, idx, next_idxs) in node_list {
            let contents: String = nodes.get(&idx).unwrap().contents.clone();

            let next_nodes: Vec<Rc<Node>> = next_idxs
                .iter()
                .map(|idx| nodes.get(idx).unwrap().clone())
                .collect();

            let new_node = Rc::new(Node {
                next_nodes,
                contents,
            });

            nodes.insert(idx, new_node);
        }

        nodes.get(&0).cloned().map(Self::new)
    }

    pub fn new<T>(head: T) -> Self
    where
        T: Into<RcNode>,
    {
        let head: RcNode = head.into();

        Self {
            head: head.clone(),
            curr: head.clone(),
        }
    }

    pub fn go(&self, direction: usize) -> Option<RcNode> {
        let next_nodes = &self.curr.next_nodes;

        next_nodes.get(direction).cloned()
    }

    pub fn next(&self) -> Option<RcNode> {
        let next_nodes = &self.curr.next_nodes;

        if next_nodes.is_empty() || next_nodes.len() > 1 {
            return None;
        }

        next_nodes.get(0).cloned()
    }
}
