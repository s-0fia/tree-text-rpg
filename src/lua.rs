use crate::{get_var, set_var, LUA};
use anyhow::{Context, Result};
use mlua::prelude::*;
#[cfg(debug_assertions)]
use std::fs;

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

pub fn setup() -> Result<()> {
    let lua = LUA.lock().ok().context("Failed to get mutex lock.")?;

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
    let contents = fs::read_to_string("lua/main.lua")?;

    #[cfg(debug_assertions)]
    let globals: LuaTable = lua.load(contents).eval()?;

    #[cfg(not(debug_assertions))]
    lua.globals()
        .set("require", lua.create_function(load_mod)?)?;

    #[cfg(not(debug_assertions))]
    let globals: LuaTable = lua
        .load(
            LUA_EMBED
                .get("main")
                .context("No main file found.")?
                .clone(),
        )
        .eval()?;

    lua.globals().set("func", globals)?;

    Ok(())
}

#[macro_export]
macro_rules! call_lua_func {
    ($ret_type:ident, $func_name:expr) => {{
        let res: mlua::Result<$ret_type> = call_lua_func!($func_name);

        res
    }};
    ($func_name:expr) => {{
        use anyhow::Context;

        let lua = crate::LUA
            .lock()
            .ok()
            .context("Failed to get mutex lock.")?;

        let func: mlua::Function = lua
            .globals()
            .get::<_, mlua::Table>("func")?
            .get($func_name.to_string())?;

        func.call::<(), _>(())
    }};
}
