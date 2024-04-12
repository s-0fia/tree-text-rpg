use std::{env, fs, path::Path};

fn main() {
    if env::var("PROFILE") == Ok("release".to_string()) {
        append_lua_const();
    }
}

fn append_lua_const() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lua_embed.rs");

    let lua_dir = fs::read_dir("lua").expect("Failed to read/find lua directory");

    let mut output = String::new();

    for path in lua_dir {
        let path = path.unwrap().path();

        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext.to_str() != Some("lua") {
                continue;
            }
        } else {
            continue;
        }

        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let contents = fs::read_to_string(path)
            .expect("Failed to read file")
            .replace('{', "{{")
            .replace('}', "}}")
            .replace("\"#", "\"\\#");

        output += &format!(r##""{filename}" => r#"{contents}"#,{}"##, "\n");
    }

    fs::write(
        &dest_path,
        format!(
            r#"
use phf::phf_map;

static LUA_EMBED: phf::Map<&'static str, &'static str> = phf_map! {{
    {output}
}};
"#
        ),
    )
    .unwrap();
}
