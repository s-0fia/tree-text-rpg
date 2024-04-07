use console::Term;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, io, sync::Mutex};

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn get_var(var_name: &str) -> Option<String> {
    VARIABLES.lock().unwrap().get(var_name).cloned()
}

fn set_var(var_name: String, value: String) -> Option<String> {
    VARIABLES.lock().unwrap().insert(var_name, value)
}

fn main() {
    // println!("Hello, world!");

    process_line(String::from(r"~bar~=balls 123"));
    process_line(String::from(r"~name=r/\w+/"));
    process_line(String::from(r"~foo=bar"));
    dbg!(VARIABLES.lock().unwrap());
}

fn process_line(line: String) {
    if line.is_empty() {
        panic!("Bad empty input");
    }
    match line.chars().next().expect("Input is empty") {
        '>' => {}
        '[' => {}
        '{' => {}
        '~' => {
            variable_change(line);
        }
        _ => {
            panic!("Unexpected start of line: '{line}'");
        }
    }
}

fn text_line(line: String) {}

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
                set_var(lhs.to_string(), input.to_string());
                break;
            }
        }
    } else if fn_set.is_match(line) {
        todo!("Implement setting to output of functions")
    } else if var_val_set.is_match(line) {
        let lhs = lhs.strip_suffix('~').unwrap();
        set_var(lhs.to_string(), rhs.to_string());
    } else if var_var_set.is_match(line) {
        if let Some(value) = get_var(rhs) {
            set_var(lhs.to_string(), value);
        } else {
            panic!("Variable {rhs} is not yet set!");
        }
    }
}
