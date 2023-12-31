use std::fmt::Display;

use colored::Colorize;

pub struct Logger<'a> {
    name: &'a str,
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            LogLevel::Info => "Info".bright_black(),
            LogLevel::Warning => "Warning".yellow(),
            LogLevel::Error => "Error".red(),
        })
    }
}

impl<'a> Logger<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn info(&self, msg: String) {
        log(self.name, LogLevel::Info, msg)
    }

    pub fn error(&self, msg: String) {
        log(self.name, LogLevel::Error, msg)
    }

    pub fn warn(&self, msg: String) {
        log(self.name, LogLevel::Warning, msg)
    }
}

fn log(name: &str, lvl: LogLevel, msg: String) {
    println!("[{}/{}] {}", name, lvl, msg)
}
