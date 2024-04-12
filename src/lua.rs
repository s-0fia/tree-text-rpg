use std::fs;

use crate::{get_var, set_var, LUA};
use mlua::prelude::*;

#[cfg(not(debug_assertions))]
include!(concat!(env!("OUT_DIR"), "/lua_embed.rs"));

#[cfg(not(debug_assertions))]
fn load_mod<'a, 'b>(lua: &'a Lua, mod_name: String) -> LuaResult<LuaTable<'b>>
where
    'a: 'b,
{
    let mod_name = mod_name.strip_prefix("lua/").unwrap();
    lua.load(LUA_EMBED.get(mod_name).unwrap().to_string())
        .eval()
}

pub fn setup() -> LuaResult<()> {
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

    #[cfg(debug_assertions)]
    let contents = fs::read_to_string("lua/main.lua").expect("No ./lua/main.lua found");

    #[cfg(debug_assertions)]
    let globals: LuaTable = lua.load(contents).eval()?;

    #[cfg(not(debug_assertions))]
    lua.globals()
        .set("require", lua.create_function(load_mod)?)?;

    #[cfg(not(debug_assertions))]
    let globals: LuaTable = lua.load(LUA_EMBED.get("main").unwrap().clone()).eval()?;

    lua.globals().set("func", globals)?;

    Ok(())
}

pub fn call_func(func_name: String) -> LuaResult<String> {
    let lua = LUA.lock().unwrap();

    let func: LuaFunction = lua.globals().get::<_, LuaTable>("func")?.get(func_name)?;

    func.call::<(), String>(())
}
