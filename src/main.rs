use anyhow::Error;
use anyhow::Result;
use core::panic;
use graph::{Graph, IndexedNodeList};
use lazy_static::lazy_static;
use mlua::Lua;
use std::fs;
use std::{collections::HashMap, sync::Mutex};

use crate::graph::FileVars;

mod conditionals;
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

fn main() -> Result<()> {
    lua::setup()?;

    let file_contents = fs::read_to_string("./main.story")?;
    let (vars, lines) = FileVars::extract_from(file_contents)?;
    let new_graph = Graph::parse_string(lines)?;

    dbg!(call_lua_func!(String, "foo")?);
    dbg!(call_lua_func!(String, "bar")?);

    dbg!(&new_graph);

    unsafe {
        GRAPH = Some(new_graph);
    }

    process_lines().unwrap();

    Ok(())
}

fn process_lines() -> Result<()> {
    if unsafe { &GRAPH }.is_none() {
        return Err(Error::msg("Graph is not set yet!"));
    }

    loop {
        let line = unsafe { GRAPH.as_ref().unwrap() }
            .curr()
            .unwrap()
            .contents
            .clone();

        if line.is_empty() {
            return Err(Error::msg("Bad empty input"));
        }

        let mut idx = 0;

        match line.chars().next().expect("Input is empty") {
            '>' => text_line::process(line)?,
            '[' => {
                idx = optionals::process(line)?;
            }
            '{' => {
                idx = conditionals::process(line)?;
            }
            '~' => variable_change::process(line)?,
            _ => {
                return Err(Error::msg("Unexpected start of line"));
            }
        }

        if unsafe { GRAPH.as_mut().unwrap() }.go_mut(idx).is_none() {
            break;
        }
    }

    println!("Done!");

    Ok(())
}
