use std::env;
use std::error::Error;
use std::fs;
use std::process;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    fn parse_config() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("输入");
        }
        let query = args[1].to_string();
        let filename = args[2].to_string();
        let case_sensitive = env::var("CASE").is_err();
        println!("{}", case_sensitive);
        Ok(Config { query, filename, case_sensitive })
    }
}
pub fn run() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::parse_config().unwrap_or_else(|err| {
        println!("problem parsing args, {}", &err);
        process::exit(1);
    });
    let contents = fs::read_to_string(&config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{:?}", line);
    }
    return Ok(());
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = &query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
  Rust: 
safe, fast, productive.
  Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
