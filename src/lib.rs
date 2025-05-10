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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters");
        }

        if args.len() > 4 {
            return Err("Too many parameters");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let mut flag: Option<Flag> = None;

        if args.len() == 4 {
            let input_flag = &args[3];
            match input_flag.as_str() {
                "-i" => {
                    flag = Some(Flag::CaseInsensitive);
                }
                _ => {
                    return Err("Unsupported Flag");
                }
            }
        }

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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // contents has been linked with lifetime
    // as it is directly related to the result vec
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
