use crate::{call_lua_func, GRAPH};
use core::panic;

pub fn process(line: String) {
    if line.chars().nth(0) != Some('{') || line.chars().last() != Some('}') {
        panic!("Invalid conditional line!");
    }

    let func_name = line.strip_prefix('{').unwrap().strip_suffix("()}").unwrap();

    dbg!(&func_name);

    let value: usize = call_lua_func!(func_name).unwrap();

    let graph = unsafe { GRAPH.as_mut().unwrap() };

    if graph.go_mut(value).is_none() {
        panic!("No node at index {value}");
    }
}
