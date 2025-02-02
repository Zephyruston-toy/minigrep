use anyhow::{Ok, Result};
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// query args in docs
    #[arg(short, long)]
    pub query: String,

    /// where file
    #[arg(short, long)]
    pub file_path: String,

    /// consider case
    #[arg(short, long, env = "IGNORE_CASE", default_value_t = false)]
    pub ignore_case: bool,
}

pub fn load_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(Into::into)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>> {
    Ok(contents
        .lines()
        .filter(|line| line.contains(query))
        .collect())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>> {
    let query = query.to_lowercase();
    Ok(contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect())
}

pub fn run(args: Args) -> Result<()> {
    let path = Path::new(&args.file_path);
    let contents = load_file(path)?;
    let results = if args.ignore_case {
        search_case_insensitive(&args.query, &contents)?
    } else {
        search(&args.query, &contents)?
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}
