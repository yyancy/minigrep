use std::{env, error::Error, fs};

use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "yancy", version, about = "a simple inplementation of grep")]
pub struct Config {
    pub query: String,
    #[clap(default_value_t = String::from("poem.txt"), forbid_empty_values = true)]
    /// demo
    pub file_path: String,
    #[clap(short, long, env = "IG")]
    pub ignore_case: bool,
    #[clap(short = 'n', long)]
    pub invert: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let invert = false;
        let _args: Vec<String> = std::env::args().collect();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            invert,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config, &contents)
    } else {
        search(&config, &contents)
    };
    for line in results {
        println!("{line}")
    }
    Ok(())
}

pub fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        let contained = line.contains(&config.query);
        if contained && !config.invert {
            results.push(line);
        } else if config.invert{
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = config.query.to_lowercase();
    for line in contents.lines() {
        let contained = line.to_lowercase().contains(&query);
        if contained && !config.invert {
            results.push(line);
        } else if config.invert{
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_resut() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let config = Config {
            query: query.to_string(),
            file_path: String::new(),
            ignore_case: false,
            invert: false,
        };
        assert_eq!(vec!["safe, fast, productive."], search(&config, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let config = Config {
            query: query.to_string(),
            file_path: String::new(),
            ignore_case: false,
            invert: false,
        };
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&config, contents)
        );
    }
}
