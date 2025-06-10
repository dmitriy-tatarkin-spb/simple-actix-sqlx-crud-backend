use std::env;
use thiserror::Error;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub secret_key: String,
}

#[derive(Debug, Error)]
enum ConfigError<'a> {
    #[error("Required environment variable {0} not set.")]
    NotPresent(&'a str),
    #[error("Invalid UTF8 character in {0}.")]
    NotUnicode(&'a str),
}

pub fn load() -> Config {
    let host = process_var("HOST").unwrap();
    let port = process_var("PORT").unwrap();
    let secret_key = process_var("SECRET_KEY").unwrap();

    let Ok(port) = port.parse::<u16>() else {
        eprintln!("PORT error: Invalid value for port passed.");
        std::process::exit(1);
    };

    Config {
        host,
        port,
        secret_key,
    }
}

fn process_var(var: &str) -> anyhow::Result<String> {
    match env::var(var) {
        Ok(h) => Ok(h.to_string()),
        Err(error) => match error {
            env::VarError::NotPresent => {
                eprintln!("{}", ConfigError::NotPresent(var));
                std::process::exit(1);
            }
            env::VarError::NotUnicode(_) => {
                eprintln!("{}", ConfigError::NotUnicode(var));
                std::process::exit(1);
            }
        },
    }
}
