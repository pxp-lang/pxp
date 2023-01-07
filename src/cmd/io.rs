use colored::*;

pub fn error(message: &str) {
    eprintln!("{}", message.white().on_red().bold());
}

pub fn success(message: &str) {
    println!("{}", message.white().on_green().bold());
}