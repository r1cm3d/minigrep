use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query && self.filename == other.filename
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn config_new() {
        let args1 = &[String::from("minigrep"), String::from("query"), String::from("filename")][..];
        let args2 = &[String::from("filename")][..];
        let cases = vec![("ok", args1, Ok(Config{ query: String::from("query"), filename: String::from("filename")})),
                         ("error", args2, Err("not enough arguments"))];

        for case in cases {
            let act = Config::new(&case.1);
            let exp = case.2;

            assert_eq!(act, exp)
        }
    }
}
