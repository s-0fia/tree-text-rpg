use core::panic;
use graph::Graph;
use lazy_static::lazy_static;
use mlua::prelude::*;
use std::{
    cell::UnsafeCell,
    collections::HashMap,
    sync::{Mutex, RwLock},
};

mod graph;
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

fn setup_lua_funcs() -> LuaResult<()> {
    let lua = LUA.lock().unwrap();

    lua.globals().set(
        "g_var",
        lua.create_function(|_, var: String| -> LuaResult<String> {
            Ok(get_var(&var).unwrap_or(String::from("nil")))
        })?,
    )?;

    lua.globals().set(
        "s_var",
        lua.create_function(|_, (var, val): (String, String)| -> LuaResult<String> {
            Ok(set_var(var, val).unwrap_or(String::from("nil")))
        })?,
    )?;

    Ok(())
}

fn main() -> LuaResult<()> {
    setup_lua_funcs()?;

    let lua = LUA.lock().unwrap();
    drop(lua);

    unsafe {
        GRAPH = Some(Graph::empty());
    }
    //    process_line(String::from(r"~bar~=balls 123"));
    //    process_line(String::from(r"~name=r/\w+/"));
    //    process_line(String::from(r"~foo=bar"));
    set_var("balls", "boobs");
    set_var("guh", "Hello, world!");
    process_line(String::from(
        r">This is BRED<ITALIC<just>> a ITALIC<<plain> test",
    ));
    process_line(String::from(r">This is just a [{balls};{guh}] test."));
    process_line(String::from(r">Testing [[ ]] test"));

    Ok(())
}

fn process_line(line: String) {
    if line.is_empty() {
        panic!("Bad empty input");
    }
    match line.chars().next().expect("Input is empty") {
        '>' => text_line::process(line),
        '[' => optionals::process(line),
        '{' => {}
        '~' => variable_change::process(line),
        _ => {
            panic!("Unexpected start of line: '{line}'");
        }
    }
}

fn conditional_line(line: String) {}
