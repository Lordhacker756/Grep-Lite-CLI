use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Less arguments passed");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = args.get(3).map_or(false, |arg| arg == "--case-sensitive");

        return Ok(Config {
            query,
            file_path,
            case_sensitive,
        });
    }
}

pub fn run<'a>(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content: String = fs::read_to_string(config.file_path)?;
    let result = search(&config.query, &file_content, config.case_sensitive);

    if result.len() == 0 {
        println!("No lines found for the query provided!😔");
    } else {
        println!("Lines matched!✨");
        for (line_num, line) in &result {
            println!("ln {}: {}", line_num, line);
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str, case_sensitive: bool) -> Vec<(usize, String)> {
    let mut results: Vec<(usize, String)> = Vec::new();

    // Adjust the query based on case sensitivity
    let query = if case_sensitive {
        query.to_string()
    } else {
        query.to_lowercase()
    };

    for (i, line) in content.lines().enumerate() {
        // Adjust each line based on case sensitivity
        let line_to_search = if case_sensitive {
            line.to_string()
        } else {
            line.to_lowercase()
        };

        if line_to_search.contains(&query) {
            results.push((i + 1, line.to_string())); // Line numbers start at 1
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive_search() {
        let query = "duct";
        let content = "Hellow\nDuct tape laga le\nBwyeee";

        // Case-insensitive: "duct" matches "Duct" in line 2.
        assert_eq!(
            vec![(2, "Duct tape laga le".to_string())],
            search(&query, &content, false)
        );
    }

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let content = "Hellow\nDuct tape laga le\nBwyeee";

        // Case-sensitive: "duct" does not match "Duct", so it should return an empty vector.
        assert_eq!(
            Vec::<(usize, String)>::new(),
            search(&query, &content, true)
        );
    }
}
