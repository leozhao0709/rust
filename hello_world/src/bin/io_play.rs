use std::{fs, path::Path};

use anyhow::{Context, Ok, Result};

fn main() -> Result<()> {
    let file_path_str = "./src/bin/arr_play.rs";
    let file_path = Path::new(file_path_str);
    if file_path.exists() {
        let content = fs::read_to_string(file_path_str)
            .with_context(|| format!("read file {0} error", file_path_str))?;
        println!("{content}");
    } else {
        println!("path does not exist")
    }
    Ok(())
}
