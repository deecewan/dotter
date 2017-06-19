use ansi_term::{Style, Colour};

pub fn print_info(info: String) {
    println!("{}", info);
}

pub fn print_success(succ: String) {
    println!("{}", Style::new().fg(Colour::Green).paint(succ));
}

pub fn print_error(error: String) {
    println!("{}", Style::new().bold().fg(Colour::Red).paint(error));
}
