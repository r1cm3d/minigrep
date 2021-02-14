use std::error::Error;
use std::{fs, env};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a string query")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get filename query")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query && self.filename == other.filename
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|l| l.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contens
        .lines()
        .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }

    #[test]
    fn config_new() {
        let args1 = &[String::from("minigrep"), String::from("query"), String::from("filename")][..];
        let args2 = &[String::from("filename")][..];
        let cases = vec![("ok", args1, Ok(Config{ query: String::from("query"), filename: String::from("filename"), case_sensitive: false})),
                         ("error", args2, Err("not enough arguments"))];

        for case in cases {
            let act = Config::new(&case.1);
            let exp = case.2;

            assert_eq!(act, exp)
        }
    }
}
