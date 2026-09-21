use std::io::IsTerminal;

const GREEN_PREFIX: &str = "\x1b[32m";
const RED_PREFIX: &str = "\x1b[31m";
const ORIGIN_PREFIX: &str = "\x1b[0m";

pub fn print_colored(s: &str, color: Color) {
    if std::io::stdout().is_terminal() {
        match color {
            Color::Green => {
                println!("{}{}{}", GREEN_PREFIX, s, ORIGIN_PREFIX);
            }
            Color::Red => {
                println!("{}{}{}", RED_PREFIX, s, ORIGIN_PREFIX);
            }
        }
    } else {
        println!("{}", s);
    }
}

pub enum Color {
    Red,
    Green,
}
