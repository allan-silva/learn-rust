use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}


struct LineEntry<'a> {
    line_no: u32,
    line: &'a str,
}


impl<'a> LineEntry<'a> {
    fn new(line_no: u32, line: &str) -> LineEntry {
        LineEntry {
            line_no,
            line
        }
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let search_result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for result in search_result {
        println!("{}: {}", result.line_no, result.line);
    }
    Ok(())
}


fn search<'a>(query: &'a str, contents: &'a str) -> Vec<LineEntry<'a>> {
    let mut lines: Vec<LineEntry<'a>> = vec![];

    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            lines.push(LineEntry::new((i as u32) + 1, line));
        }
    }

    lines
}


fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<LineEntry<'a>> {
    let query = query.to_lowercase();
    let mut lines: Vec<LineEntry<'a>> = vec![];

    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            lines.push(LineEntry::new((i as u32) + 1, line));
        }
    }

    lines
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

        let search_result = search(query, contents)
            .iter()
            .map(|line_entry| line_entry.line)
            .collect::<Vec<_>>();

        assert_eq!(
            vec!["safe, fast, productive."],
            search_result);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        let search_result = search_case_insensitive(query, contents)
            .iter()
            .map(|line_entry| line_entry.line)
            .collect::<Vec<_>>();

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_result
        )
    }
}