#![allow(dead_code)]
use anyhow::{Context, Result};
use std::{num::ParseIntError, time::Duration};

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

pub struct FileVars {
    pub version: String,
    pub write_speed: Duration,
    pub write_type: WriteType,
}

pub enum WriteType {
    Char,
    Word,
    Line,
}

impl FileVars {
    pub fn extract_from(lines: String) -> Result<(Self, Vec<String>)> {
        let mut lines = lines.lines();
        let vars = lines.next().context("Lines is empty.")?.split_whitespace();

        let mut version = None;
        let mut write_speed = None;
        let mut write_type = None;

        for v in vars {
            let v = v.to_lowercase();
            let (name, val) = v
                .split_once('=')
                .context("No equals seperator token in variable")?;

            match name {
                "version" => {
                    version = match val {
                        "0.1" => Some("0.1".into()),
                        _ => None,
                    }
                }
                "write_speed" => {
                    write_speed = if val.contains("ms") {
                        val.parse().ok().map(|millis| Duration::from_millis(millis))
                    } else if val.contains("s") {
                        val.parse().ok().map(|secs| Duration::from_secs(secs))
                    } else {
                        None
                    }
                }
                "write_type" => {
                    write_type = match val {
                        "char" => Some(WriteType::Char),
                        "word" => Some(WriteType::Word),
                        "line" => Some(WriteType::Line),
                        _ => None,
                    }
                }
                _ => {}
            }
        }

        let version = version.context("Invalid/no version given!")?;
        let write_speed = write_speed.unwrap_or(Duration::from_millis(100));
        let write_type = write_type.unwrap_or(WriteType::Line);

        let file_vars = Self {
            version,
            write_speed,
            write_type,
        };

        Ok((file_vars, lines.map(|l| l.to_string()).collect()))
    }
}

impl Graph {
    pub fn parse_string(lines: Vec<String>) -> Result<Self> {
        let mut graph = Vec::with_capacity(lines.len());

        for line in lines {
            let (next_nodes, contents) = line
                .split_once(";")
                .context("No semicolon seperator in line.")?;

            let next_nodes: Vec<usize> = if next_nodes.is_empty() {
                vec![]
            } else {
                next_nodes
                    .split(',')
                    .map(|s| s.parse())
                    .collect::<Result<_, ParseIntError>>()?
            };

            let contents = contents.to_string();

            let node = Node {
                next_nodes,
                contents,
            };

            graph.push(node);
        }

        Ok(Self::new(graph))
    }

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
