use crate::get_var;
use anyhow::{Context, Error, Result};
use console::Style;
use core::panic;
use rand::seq::SliceRandom;

fn simple_open_close_line(
    line: String,
    (start, end): (char, char),
    success: fn(String) -> Result<String>,
) -> Result<String> {
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
                output += &success(temp_buf)?;
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

    return Ok(output);
}

pub fn process(line: String) -> Result<()> {
    let line = line
        .strip_prefix('>')
        .context("Failed to remove > prefix from text line.")?
        .to_string();

    let rand_line_fn = |temp_buf: String| {
        let options: Vec<&str> = temp_buf.split(';').collect();

        if options.is_empty() {
            return Err(Error::msg("Invalid random selector, no elements."));
        }

        let mut rng = rand::thread_rng();

        Ok(options.choose(&mut rng).unwrap().to_string())
    };

    let var_line_fn = |temp_buf: String| {
        let fields: Vec<&str> = temp_buf.split(';').collect();

        match fields.len() {
            1 => get_var(fields[0]).context("Failed to get variable value"),
            2 => {
                let value = get_var(fields[1]).context("Failed to get variable value")?;

                let processed = match fields[0] {
                    "U" => value.to_uppercase(),
                    "l" => value.to_lowercase(),
                    "F" => format!("{}{}", &value[0..1].to_uppercase(), &value[1..]),
                    "f" => format!("{}{}", &value[0..1].to_lowercase(), &value[1..]),
                    f => Err(Error::msg(format!("Unrecognised post process {f}")))?,
                };

                Ok(processed)
            }
            _ => Err(Error::msg("Too many/no fields given in variable")),
        }
    };

    let output = simple_open_close_line(line, ('[', ']'), rand_line_fn)?;
    let mut output = simple_open_close_line(output, ('{', '}'), var_line_fn)?;

    const STYLES: [&'static str; 17] = [
        "BOLD",
        "DIM",
        "ITALIC",
        "UNDERLINED",
        "BLINK",
        "BLINKFAST",
        "REVERSE",
        "HIDDEN",
        "STRIKETHROUGH",
        "BLACK",
        "BLUE",
        "GREEN",
        "RED",
        "CYAN",
        "MAGENTA",
        "YELLOW",
        "WHITE",
    ];

    for style in STYLES.iter() {
        let mut next = output.find(&format!("{style}<"));

        while let Some(idx) = next {
            let mut new_out = String::new();

            let mut out_chars = output.chars();

            if output.chars().nth(idx + style.len() + 1) == Some('<') {
                output.remove(idx + style.len() + 1);
                next = output[idx + style.len() + 1..].find(&format!("{style}<"));
                continue;
            }

            let fg_or_bg_colours = if output.chars().nth(idx - 1) == Some('B') {
                new_out += &output[..idx - 1];
                [
                    Style::on_black,
                    Style::on_blue,
                    Style::on_green,
                    Style::on_red,
                    Style::on_cyan,
                    Style::on_magenta,
                    Style::on_yellow,
                    Style::on_white,
                ]
            } else {
                new_out += &output[..idx];
                [
                    Style::black,
                    Style::blue,
                    Style::green,
                    Style::red,
                    Style::cyan,
                    Style::magenta,
                    Style::yellow,
                    Style::white,
                ]
            };

            let mut close_idx = output[idx..]
                .find('>')
                .context("No valid closing tag for style.")?
                + idx;

            while out_chars.nth(close_idx + 1) == Some('>') {
                close_idx += 1;
            }

            let apply_style_func = |func: fn(Style) -> Style| {
                func(Style::new())
                    .apply_to(&output[idx + style.len() + 1..close_idx])
                    .to_string()
            };

            new_out += &match *style {
                "BOLD" => apply_style_func(Style::bold),
                "DIM" => apply_style_func(Style::dim),
                "ITALIC" => apply_style_func(Style::italic),
                "UNDERLINED" => apply_style_func(Style::underlined),
                "BLINK" => apply_style_func(Style::blink),
                "BLINKFAST" => apply_style_func(Style::blink_fast),
                "REVERSE" => apply_style_func(Style::reverse),
                "HIDDEN" => apply_style_func(Style::hidden),
                "STRIKETHROUGH" => apply_style_func(Style::strikethrough),
                "BLACK" => apply_style_func(fg_or_bg_colours[0]),
                "BLUE" => apply_style_func(fg_or_bg_colours[1]),
                "GREEN" => apply_style_func(fg_or_bg_colours[2]),
                "RED" => apply_style_func(fg_or_bg_colours[3]),
                "CYAN" => apply_style_func(fg_or_bg_colours[4]),
                "MAGENTA" => apply_style_func(fg_or_bg_colours[5]),
                "YELLOW" => apply_style_func(fg_or_bg_colours[6]),
                "WHITE" => apply_style_func(fg_or_bg_colours[7]),
                _ => Err(Error::msg("Invalid style."))?,
            };

            new_out += &output[close_idx + 1..];
            output = new_out;

            next = output.find(&format!("{style}<"));
        }
    }

    println!("{}", output);

    Ok(())
}
