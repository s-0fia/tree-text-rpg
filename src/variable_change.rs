use crate::{call_lua_func, get_var, set_var};
use core::panic;
use regex::Regex;
use std::io;

pub fn process(line: String) {
    if !line.contains('=') {
        panic!("Invalid variable assignment");
    }

    let line = line.strip_prefix('~').unwrap();

    let input = Regex::new(r"^[a-z_]+=r\/.+\/$").unwrap();
    let var_val_set = Regex::new(r"^[a-z_]+~=.+$").unwrap();
    let var_var_set = Regex::new(r"^[a-z_]+=[a-z_]+$").unwrap();
    let fn_set = Regex::new(r"^[a-z_]+=[a-z_]+\([a-z_]*\)$").unwrap();

    let (lhs, rhs) = line.split_once('=').unwrap();

    if input.is_match(line) {
        let pat = format!("^{}$", &rhs[2..rhs.len() - 1]);
        let pat = Regex::new(&pat).unwrap_or_else(|_| panic!("Invalid regex pattern: {pat}!"));

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if pat.is_match(input) {
                set_var(lhs, input);
                break;
            }
        }
    } else if fn_set.is_match(line) {
        let func_name = rhs.strip_suffix("()").unwrap();
        let value: String = call_lua_func!(func_name).unwrap();
        set_var(lhs, value);
    } else if var_val_set.is_match(line) {
        let lhs = lhs.strip_suffix('~').unwrap();
        set_var(lhs, rhs);
    } else if var_var_set.is_match(line) {
        if let Some(value) = get_var(rhs) {
            set_var(lhs, value);
        } else {
            panic!("Variable {rhs} is not yet set!");
        }
    }
}
