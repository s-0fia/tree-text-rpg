use crate::{call_lua_func, get_var, set_var};
use anyhow::{Context, Error, Result};
use regex::Regex;
use std::io;

pub fn process(line: String) -> Result<()> {
    if !line.contains('=') {
        return Err(Error::msg("Invalid variable assignment, no equals sign."));
    }

    let line = line
        .strip_prefix('~')
        .context("Invalid variable change line, no tilde prefix.")?;

    let input = Regex::new(r"^[a-z_]+=r\/.+\/$")?;
    let var_val_set = Regex::new(r"^[a-z_]+~=.+$")?;
    let var_var_set = Regex::new(r"^[a-z_]+=[a-z_]+$")?;
    let fn_set = Regex::new(r"^[a-z_]+=[a-z_]+\([a-z_]*\)$")?;

    let (lhs, rhs) = line
        .split_once('=')
        .context("Invalid variable assignment, no equals sign.")?;

    if input.is_match(line) {
        let pat = format!("^{}$", &rhs[2..rhs.len() - 1]);
        let pat = Regex::new(&pat)?;

        dbg!(&pat);

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if pat.is_match(input) {
                set_var(lhs, input);
                break;
            }
        }

        return Ok(());
    } else if fn_set.is_match(line) {
        let func_name = rhs.strip_suffix("()").context(
            "Invalid function in variable assignemnt, brackets do not suffix the function name",
        )?;
        let value: String = call_lua_func!(func_name)?;

        set_var(lhs, value);

        return Ok(());
    } else if var_val_set.is_match(line) {
        let lhs = lhs
            .strip_suffix('~')
            .context("Invalid variable to value assignenment, no ~= found.")?;

        set_var(lhs, rhs);

        return Ok(());
    } else if var_var_set.is_match(line) {
        if let Some(value) = get_var(rhs) {
            set_var(lhs, value);
        } else {
            return Err(Error::msg(format!("Variable {rhs} is not yet set!")));
        }

        return Ok(());
    }

    Err(Error::msg("Invalid var line input!"))
}
