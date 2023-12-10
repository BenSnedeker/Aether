use std::fmt;
use aether_common::{say, hey, nay, yay};
use better_term::Color;

pub fn _say_client(ip: &str, args: fmt::Arguments) {
    say!("{}{}{}: {}{}", Color::White, ip, Color::BrightBlack, Color::BrightWhite, args);
}

#[macro_export]
macro_rules! say_client {
    ($ip:expr, $($arg:tt)*) => {{
        $crate::log::_say_client($ip, format_args!($($arg)*));
    }}
}

pub fn _hey_client(ip: &str, args: fmt::Arguments) {
    hey!("{}{}{}: {}{}", Color::White, ip, Color::BrightBlack, Color::BrightYellow, args);
}

#[macro_export]
macro_rules! hey_client {
    ($ip:expr, $($arg:tt)*) => {{
        $crate::log::_hey_client($ip, format_args!($($arg)*));
    }}
}

pub fn _yay_client(ip: &str, args: fmt::Arguments) {
    yay!("{}{}{}: {}{}", Color::White, ip, Color::BrightBlack, Color::BrightGreen, args);
}

#[macro_export]
macro_rules! yay_client {
    ($ip:expr, $($arg:tt)*) => {{
        $crate::log::_yay_client($ip, format_args!($($arg)*));
    }}
}

pub fn _nay_client(ip: &str, args: fmt::Arguments) {
    nay!("{}{}{}: {}{}", Color::White, ip, Color::BrightBlack, Color::BrightRed, args);
}

#[macro_export]
macro_rules! nay_client {
    ($ip:expr, $($arg:tt)*) => {{
        $crate::log::_nay_client($ip, format_args!($($arg)*));
    }}
}