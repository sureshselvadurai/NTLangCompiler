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
}
