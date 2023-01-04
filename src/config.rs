use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::read();
}

#[derive(Debug)]
pub struct Config {}

impl Config {
    pub fn read() -> Self {
        Self {}
    }
}
