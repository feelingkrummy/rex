
enum ConfigError {
    NotEnoughArgs,
    UnexpectedEndOfArgs,
    FailedToParseColNum,
}

struct Config {
    pub file_path: String,
    pub cols: u64,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, ConfigError> {
        let mut config = Config{
            cols: 8,
            file_path: String::new(),
        };

        if args.len() < 2 {
            return Err(ConfigError::NotEnoughArgs);
        }

        let mut args_iter = args.iter();
    
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-c" => {
                    let next_arg = args_iter.next();
                    match next_arg {
                        Some(col_str) => {
                            match col_str.parse::<u64>() {
                                Ok(col) => {
                                    config.cols = col;
                                }
                                Err(_) => return Err(ConfigError::FailedToParseColNum),
                            }
                        }
                        None => return Err(ConfigError::UnexpectedEndOfArgs),
                    }
                }
                &_ => todo!()
            }
        }

        return Ok(config);
    }
}
