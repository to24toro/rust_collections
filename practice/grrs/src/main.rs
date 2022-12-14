// #![allow(unused)]

// use clap::Parser;

// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     #[clap(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

// fn main() {
//     let args = Cli::parse();
//     let content = std::fs::read_to_string(&args.path).expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "text.txt";
    let content = std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}