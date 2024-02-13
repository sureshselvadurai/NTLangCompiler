pub struct Config {
    pub expression: String,
    pub base: u32,
    pub width: u32,
    pub unsigned_int: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            expression: String::new(),
            base: 10,
            width: 32,
            unsigned_int: true,
        }
    }

    pub fn parse_args(args: Vec<String>) -> Config {
        let mut config = Config::new();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-e" => {
                    if i + 1 < args.len() {
                        config.expression = args[i + 1].clone();
                        i += 1;
                    } else {
                        println!("Error: Missing argument for -e");
                        return Config::new(); // Return default config on error
                    }
                }
                "-b" => {
                    if i + 1 < args.len() {
                        if let Ok(val) = args[i + 1].parse::<u32>() {
                            config.base = val;
                            i += 1;
                        } else {
                            println!("Error: Invalid argument for -b");
                            return Config::new(); // Return default config on error
                        }
                    } else {
                        println!("Error: Missing argument for -b");
                        return Config::new(); // Return default config on error
                    }
                }
                "-w" => {
                    if i + 1 < args.len() {
                        if let Ok(val) = args[i + 1].parse::<u32>() {
                            config.width = val;
                            i += 1;
                        } else {
                            println!("Error: Invalid argument for -w");
                            return Config::new(); // Return default config on error
                        }
                    } else {
                        println!("Error: Missing argument for -w");
                        return Config::new(); // Return default config on error
                    }
                }
                _ => {
                    println!("Error: Unknown argument {}", args[i]);
                    return Config::new(); // Return default config on error
                }
            }
            i += 1;
        }
        config
    }
}
