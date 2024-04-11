use core::panic;
use std::io;

use crate::GRAPH;

pub fn process(line: String) {
    if line.chars().nth(0) != Some('[') || line.chars().last() != Some(']') {
        panic!("Invalid optional line!");
    }

    let line = line.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
    let options: Vec<&str> = line.split(';').collect();
    dbg!(&options);

    let graph = unsafe { &mut GRAPH }.as_mut().expect("No graph defined!");

    if graph.next_nodes_len().unwrap() > options.len() {
        panic!("Too few options given in optional '{line}'");
    } else if graph.next_nodes_len().unwrap() < options.len() {
        panic!("Too many options given in option '{line}'");
    }

    let choice_idx = loop {
        let mut buf = String::new();

        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line...");

        let input = buf.trim();

        if let Some(idx) = options.iter().position(|&o| o == input) {
            break idx;
        }
    };

    if graph.go_mut(choice_idx).is_none() {
        panic!("No node at index {choice_idx}");
    }
}
