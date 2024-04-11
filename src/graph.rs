#![allow(dead_code)]
use std::{collections::HashMap, rc::Rc};
pub struct Node {
    pub next_nodes: Vec<RcNode>,
    pub contents: String,
}

impl Node {
    #[inline]
    pub fn new() -> Self {
        Node {
            next_nodes: vec![],
            contents: String::new(),
        }
    }
}

pub type RcNode = Rc<Node>;

pub struct Graph {
    head: RcNode,
    curr: RcNode,
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

    #[inline]
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

    #[inline]
    pub fn empty() -> Self {
        Self::new(Node::new())
    }

    #[inline]
    pub fn go(&self, direction: usize) -> Option<RcNode> {
        let next_nodes = &self.curr.next_nodes;

        next_nodes.get(direction).cloned()
    }

    #[inline]
    pub fn go_mut(&mut self, direction: usize) -> Option<RcNode> {
        if let Some(node) = self.go(direction) {
            self.curr = node.clone();
            Some(node)
        } else {
            None
        }
    }

    #[inline]
    pub fn next(&self) -> Option<RcNode> {
        let next_nodes = &self.curr.next_nodes;

        if next_nodes.is_empty() || next_nodes.len() > 1 {
            return None;
        }

        next_nodes.get(0).cloned()
    }

    #[inline]
    pub fn next_mut(&mut self) -> Option<RcNode> {
        if let Some(node) = self.next() {
            self.curr = node.clone();
            Some(node)
        } else {
            None
        }
    }

    #[inline]
    pub fn next_nodes_len(&self) -> usize {
        self.curr.next_nodes.len()
    }
}
