//! https://doc.rust-lang.org/book/ch12-00-an-io-project.html

use std::error::Error;

#[derive(Debug)]
struct Configure {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Configure {
    fn build(mut it: impl Iterator<Item=String>) -> Result<Configure, &'static str> {
        it.next();
        let query = match it.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),
        };
        let file_path = match it.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path!"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Configure {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    search(&query.to_lowercase(), content)
}

pub fn run(it: impl Iterator<Item=String>) -> Result<(), Box<dyn Error>> {
    let config = Configure::build(it)?;
    dbg!(&config);
    let contents = std::fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}