use std::error::Error;
use std::fs;

pub enum Flag {
    CaseInsensitive,
}

pub struct Config {
    pub query: String, 
    pub file_path: String,
    pub flag: Option<Flag>
}


impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // ignore the binary file path

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Could not read query")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Could not read file path")
        };

        let flag = match args.next() {
            Some(arg) => {
                match arg.as_str() {
                    "-i" => Some(Flag::CaseInsensitive),
                    _ => return Err("Unsupported Flag")
                }
            },
            None => None
        };

        if args.next().is_some() { return Err("Unsupported parameters supplied") }

        Ok( Config {
            query,
            file_path,
            flag
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = match config.flag {
        Some(Flag::CaseInsensitive) => search_case_insensitive(&config.query, &contents),
        None => search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // contents has been linked with lifetime
    // as it is directly related to the result vec
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // contents has been linked with lifetime
    // as it is directly related to the result vec
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
