use core::panic;
use graph::{Graph, IndexedNodeList};
use lazy_static::lazy_static;
use mlua::Lua;
use mlua::Result as LuaResult;
use std::{collections::HashMap, sync::Mutex};

mod graph;
mod lua;
mod optionals;
mod text_line;
mod variable_change;

static mut GRAPH: Option<Graph> = None;

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref LUA: Mutex<Lua> = Mutex::new(Lua::new());
}

#[inline]
fn get_var(var_name: &str) -> Option<String> {
    VARIABLES.lock().unwrap().get(var_name).cloned()
}

#[inline]
fn set_var<S: Into<String>, T: Into<String>>(var_name: S, value: T) -> Option<String> {
    VARIABLES
        .lock()
        .unwrap()
        .insert(var_name.into(), value.into())
}

fn main() -> LuaResult<()> {
    lua::setup()?;

    dbg!(lua::call_func("foo".to_string())?);
    dbg!(lua::call_func("bar".to_string())?);

    let idx_node_list: IndexedNodeList = vec![
        (0, ">Please input a number 1-3:".into(), vec![1]),
        (1, "[1;2;3]".into(), vec![2, 3, 4]),
        (2, ">You selected 1!".into(), vec![]),
        (3, ">You selected 2!".into(), vec![]),
        (4, ">You selected 3!".into(), vec![]),
    ];

    let new_graph = Graph::parse(idx_node_list);

    dbg!(&new_graph);

    unsafe {
        GRAPH = Some(new_graph);
    }

    process_lines();

    Ok(())
}

fn process_lines() {
    if unsafe { &GRAPH }.is_none() {
        panic!("Graph is not set yet!");
    }
    loop {
        let line = unsafe { GRAPH.as_ref().unwrap() }
            .curr()
            .unwrap()
            .contents
            .clone();

        if line.is_empty() {
            panic!("Bad empty input");
        }

        match line.chars().next().expect("Input is empty") {
            '>' => text_line::process(line),
            '[' => {
                optionals::process(line);
                continue;
            }
            '{' => {}
            '~' => variable_change::process(line),
            _ => {
                panic!("Unexpected start of line: '{line}'");
            }
        }

        if unsafe { GRAPH.as_mut().unwrap() }.next_mut().is_none() {
            break;
        }
    }

    println!("Done!");
}

fn conditional_line(line: String) {}
