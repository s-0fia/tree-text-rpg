#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Node {
    pub next_nodes: Vec<usize>,
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

#[derive(Debug)]
pub struct Graph {
    nodes: GraphList,
    curr: usize,
}

pub type GraphList = Vec<Node>;
pub type IndexedNodeList = Vec<(usize, String, Vec<usize>)>;

impl Graph {
    pub fn parse(mut node_list: IndexedNodeList) -> Self {
        node_list.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

        let mut graph = Vec::with_capacity(node_list.len());

        for (_, contents, next_nodes) in node_list {
            graph.push(Node {
                next_nodes,
                contents,
            });
        }

        Self::new(graph)
    }

    #[inline]
    pub fn new(nodes: GraphList) -> Self {
        Self { nodes, curr: 0 }
    }

    #[inline]
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    #[inline]
    pub fn go(&self, direction: usize) -> Option<usize> {
        self.nodes
            .get(self.curr)?
            .next_nodes
            .get(direction)
            .copied()
    }

    #[inline]
    pub fn go_mut(&mut self, direction: usize) -> Option<usize> {
        let idx = self.go(direction)?;

        self.curr = idx;

        Some(idx)
    }

    #[inline]
    pub fn next(&self) -> Option<usize> {
        self.nodes.get(self.curr)?.next_nodes.get(0).copied()
    }

    #[inline]
    pub fn next_mut(&mut self) -> Option<usize> {
        let idx = self.next()?;

        self.curr = idx;

        Some(idx)
    }

    #[inline]
    pub fn next_nodes_len(&self) -> Option<usize> {
        Some(self.nodes.get(self.curr)?.next_nodes.len())
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Option<Node> {
        Some(self.nodes.get(idx)?.clone())
    }

    #[inline]
    pub fn curr(&self) -> Option<Node> {
        Some(self.get(self.curr)?.clone())
    }
}
