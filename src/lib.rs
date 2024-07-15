use regex::Regex;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Params {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Params {
    pub fn build(args: &Vec<String>) -> Result<Params, &str> {
        let mut query = "";
        let mut file_path = "";
        let mut ignore_case = false;
        let query_regex = Regex::new("^-q=(.*)").unwrap();
        let file_path_regex = Regex::new("^-f=(.*)").unwrap();
        for arg in &args[1..] {
            match arg.as_str() {
                "-i" => {
                    ignore_case = true;
                }
                _ => {
                    if let Some(captures) = query_regex.captures(arg) {
                        query = captures.get(1).unwrap().as_str();
                    }
                    if let Some(captures) = file_path_regex.captures(arg) {
                        file_path = captures.get(1).unwrap().as_str();
                    }
                }
            }
        }
        if query.is_empty() || file_path.is_empty() {
            return Err("Not enough valid param");
        }
        Ok(Params {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })
    }
}

pub fn run(params: &Params) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&params.file_path)?;
    if params.ignore_case {
        for (index, line) in search_case_insensitive(&params.query, &contents) {
            println!("row[{}]:{}", index, line)
        }
    } else {
        for (index, line) in search(&params.query, &contents) {
            println!("row[{}]:{}", index, line)
        }
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = vec![];
    for (index, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((index, line));
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = vec![];
    let query = query.to_lowercase();
    for (index, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((index, line));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_build() {
        let params = vec![
            "".to_string(),
            "-i".to_string(),
            "-q=query".to_string(),
            "-f=test.txt".to_string(),
        ];
        let params_struct = Params::build(&params);
        assert_eq!(
            params_struct,
            Ok(Params {
                query: "query".to_string(),
                file_path: "test.txt".to_string(),
                ignore_case: true
            })
        )
    }

    #[test]
    fn no_enough_param() {
        let params = vec!["".to_string(), "-i".to_string(), "-q=query".to_string()];
        let params_struct = Params::build(&params);
        assert_eq!(params_struct, Err("Not enough valid param"))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(1, search(query, contents).get(0).unwrap().0);
        assert_eq!(
            "safe, fast, productive.",
            search(query, contents).get(0).unwrap().1
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
        assert_eq!(2, search_case_insensitive(query, contents).len());
        assert_eq!(
            0,
            search_case_insensitive(query, contents).get(0).unwrap().0
        );
        assert_eq!(
            "Rust:",
            search_case_insensitive(query, contents).get(0).unwrap().1
        );
        assert_eq!(
            3,
            search_case_insensitive(query, contents).get(1).unwrap().0
        );
        assert_eq!(
            "Trust me.",
            search_case_insensitive(query, contents).get(1).unwrap().1
        );
    }
}
