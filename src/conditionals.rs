use crate::call_lua_func;
use anyhow::{Context, Result};

pub fn process(line: String) -> Result<usize> {
    let func_name = line
        .strip_prefix('{')
        .context("No conditional line prefix {")?
        .strip_suffix("()}")
        .context("No conditional line suffix }")?;

    let value: usize = call_lua_func!(func_name)?;

    Ok(value)
}
