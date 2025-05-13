use std::{env, error::Error, fs::File, io::{self, Read}};

pub struct Config {
    query: String,
    filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        if args.len() >= 3 {
            let filename = args[2].clone();
        } else {
            println!("pipe")
        }

        // let mut stdin = io::stdin();
        // let mut stdin_buffer = [0; 1];

        // match stdin.read(&mut stdin_buffer) {
        //     Ok(0) => println!("tme"),
        //     Ok(_) => println!("nao tem"),
        //     Err(_) => println!("nao tem 2")
        // }

        let query = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = !env::var("CASE_INSENSITIVE").is_err();

        if args.len() >= 4 && args[3] == "true" {
            case_sensitive = true;
        }

        Ok(Config { query, filename, case_sensitive })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

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


// Precisamos colocar o lifetime aqui porque o retorno dessa funcao vai ter relacao direta com a contents
// ja que vamos colocar dentro de Vec a referencai para strings dentro da contetns, entao o lifetime delas precisa ser igual
// Se o contents sair do escopo a vec tbm precisa sair
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

        #[test]
    fn case_sensitive() {
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
