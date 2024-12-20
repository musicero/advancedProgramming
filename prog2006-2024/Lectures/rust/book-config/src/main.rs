
use std::env;

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

/* Original implementation
    Rust book: chapter 13, Listing 13-17

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
*/

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();
        Ok(Config {
            query: args.next().ok_or("Didn't get a query string")?,
            file_path: args.next().ok_or("Didn't get a file path")?,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}


fn main() {
    println!("Parse command line arguments and config");
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        panic!("Problem parsing arguments: {}", err);
    });
    println!("Query: {:?}", config);
}
