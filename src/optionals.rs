use anyhow::{Context, Error, Result};
use std::{cmp::Ordering, io};

use crate::GRAPH;

pub fn process(line: String) -> Result<usize> {
    let line = line
        .strip_prefix('[')
        .context("No [ prefix on optional line!")?
        .strip_suffix(']')
        .context("No ] suffix on optional line")?;

    let options: Vec<&str> = line.split(';').collect();

    let graph = unsafe { &mut GRAPH }
        .as_mut()
        .context("No graph defined!")?;

    let _ = match graph
        .next_nodes_len()
        .context("Failed to read next nodes length.")?
        .cmp(&options.len())
    {
        Ordering::Less => Err(Error::msg("Too few options given in optional")),
        Ordering::Greater => Err(Error::msg("Too many options given in optional")),
        _ => Ok(()),
    }?;

    let choice_idx = loop {
        let mut buf = String::new();

        io::stdin().read_line(&mut buf)?;

        let input = buf.trim();

        if let Some(idx) = options.iter().position(|&o| o == input) {
            break idx;
        }
    };

    Ok(choice_idx)
}
