use std::env;
use std::error::Error;
use std::fs;

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

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip program name

        // Return an error instead of panicking to avoid showing extraneous
        // backtrace information to users. This is an *expected* error and should
        // be handled as such.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // env::var() returns error if unset -> use .is_err() to get a bool
        // TODO: Use CLI arg to set this instead of/in addition to env var
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
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
        let query = "hid";
        let contents = "\
A box without hinges, key, or lid,
Yet golden treasure inside is hid.";

        assert_eq!(
            vec!["Yet golden treasure inside is hid."],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "An";
        let contents = "\
An eye in a blue face
Saw an eye in a green face.
'That eye is like to this eye'
Said the first eye,
'But in low place,
Not in high place.'";

        assert_eq!(vec!["An eye in a blue face"], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "An";
        let contents = "\
An eye in a blue face
Saw an eye in a green face.
'That eye is like to this eye'
Said the first eye,
'But in low place,
Not in high place.'";

        assert_eq!(
            vec!["An eye in a blue face", "Saw an eye in a green face."],
            search_case_insensitive(query, contents)
        )
    }
}
