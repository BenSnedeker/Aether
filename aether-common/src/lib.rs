use better_term::Color;
use std::fmt;

fn raw_log(prefix: String, msg_color: Color, args: fmt::Arguments) {
    println!(
        "{b}[{}{b}] {}{}",
        prefix,
        msg_color,
        args,
        b = Color::BrightBlack
    )
}

pub fn say(args: fmt::Arguments) {
    raw_log(format!("{}:", Color::White), Color::BrightWhite, args);
}

#[macro_export]
macro_rules! say {
    ($($arg:tt)*) => { $crate::say(format_args!($($arg)*)) }
}

pub fn yay(args: fmt::Arguments) {
    raw_log(format!("{}✔", Color::Green), Color::BrightGreen, args);
}

#[macro_export]
macro_rules! yay {
    ($($arg:tt)*) => { $crate::yay(format_args!($($arg)*)) }
}

pub fn hey(args: fmt::Arguments) {
    raw_log(format!("{}!", Color::Yellow), Color::BrightYellow, args);
}

#[macro_export]
macro_rules! hey {
    ($($arg:tt)*) => { $crate::hey(format_args!($($arg)*)) }
}

pub fn nay(args: fmt::Arguments) {
    raw_log(format!("{}✘", Color::Red), Color::BrightRed, args);
}

#[macro_export]
macro_rules! nay {
    ($($arg:tt)*) => { $crate::nay(format_args!($($arg)*)) }
}
