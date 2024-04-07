use console::Term;
use lazy_static::lazy_static;
use mlua::prelude::*;
use rand::seq::SliceRandom;
use regex::Regex;
use std::{collections::HashMap, fmt::format, io, sync::Mutex};

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref LUA: Mutex<Lua> = Mutex::new(Lua::new());
}

fn get_var(var_name: &str) -> Option<String> {
    VARIABLES.lock().unwrap().get(var_name).cloned()
}

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

    //    process_line(String::from(r"~bar~=balls 123"));
    //    process_line(String::from(r"~name=r/\w+/"));
    //    process_line(String::from(r"~foo=bar"));
    process_line(String::from(r">This is just a plain test"));
    process_line(String::from(
        r">This is just a [rand;random;123;sdhjkf;test thjkdjk] test.",
    ));
    process_line(String::from(r">Testing [[ ]] test"));

    Ok(())
}

fn process_line(line: String) {
    if line.is_empty() {
        panic!("Bad empty input");
    }
    match line.chars().next().expect("Input is empty") {
        '>' => text_line(line),
        '[' => {}
        '{' => {}
        '~' => variable_change(line),
        _ => {
            panic!("Unexpected start of line: '{line}'");
        }
    }
}

fn simple_open_close_line(
    line: String,
    (start, end): (char, char),
    success: fn(String) -> String,
) -> String {
    let mut escaped_char = false;
    let mut in_block_seq = false;
    let mut output = String::new();
    let mut temp_buf = String::new();

    for (i, c) in line.chars().enumerate() {
        if escaped_char {
            output.push(c);
            escaped_char = false;
            continue;
        }

        if c == start {
            escaped_char = line.chars().nth(i + 1) == Some(start);

            if !escaped_char {
                in_block_seq = true;
            }

            continue;
        }

        if c == end {
            escaped_char = line.chars().nth(i + 1) == Some(end);

            if !escaped_char && in_block_seq {
                output += &success(temp_buf);
                temp_buf = String::new();

                in_block_seq = false;
            } else if !escaped_char && !in_block_seq {
                panic!("Unclosed bracket in line!");
            }

            continue;
        }

        if in_block_seq {
            temp_buf.push(c);
        } else {
            output.push(c);
        }
    }

    return output;
}

fn text_line(line: String) {
    let rand_line_fn = |temp_buf: String| {
        let options: Vec<&str> = temp_buf.split(';').collect();
        if options.is_empty() {
            panic!("Invalid random selector, no elements");
        }
        let mut rng = rand::thread_rng();

        options.choose(&mut rng).unwrap().to_string()
    };

    let var_line_fn = |temp_buf: String| {
        let fields: Vec<&str> = temp_buf.split(';').collect();
        match fields.len() {
            1 => {
                return get_var(fields[0]).unwrap();
            }
            2 => {
                let value = get_var(fields[1]).unwrap();

                let processed = match fields[0] {
                    "U" => value.to_uppercase(),
                    "l" => value.to_lowercase(),
                    "F" => format!("{}{}", &value[0..1].to_uppercase(), &value[1..]),
                    "f" => format!("{}{}", &value[0..1].to_lowercase(), &value[1..]),
                    f => panic!("Unrecognised post process {f}"),
                };

                return processed;
            }
            _ => panic!("Too many/little fields given in variable"),
        }
    };

    let output = simple_open_close_line(line, ('[', ']'), rand_line_fn);
    let output = simple_open_close_line(output, ('{', '}'), var_line_fn);

    println!("{}", output);
}

fn option_line(line: String) {}

fn conditional_line(line: String) {}

fn variable_change(line: String) {
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
        todo!("Implement setting to output of functions")
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
