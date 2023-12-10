use better_term::{Color, flush_styles};
use std::fmt;

pub mod change;

fn raw_log(prefix: String, msg_color: Color, args: fmt::Arguments) {
    println!(
        "{b}[{}{b}] {}{}",
        //"{} {}{}",
        prefix,
        msg_color,
        args,
        b = Color::BrightBlack
    );
    flush_styles();
}

pub fn _say(args: fmt::Arguments) {
    raw_log(format!("{}#", Color::White), Color::BrightWhite, args);
}

#[macro_export]
macro_rules! say {
    ($($arg:tt)*) => { $crate::_say(format_args!($($arg)*)) }
}

pub fn _yay(args: fmt::Arguments) {
    raw_log(format!("{}✔", Color::Green), Color::BrightGreen, args);
}

#[macro_export]
macro_rules! yay {
    ($($arg:tt)*) => { $crate::_yay(format_args!($($arg)*)) }
}

pub fn _hey(args: fmt::Arguments) {
    raw_log(format!("{}!", Color::Yellow), Color::BrightYellow, args);
}

#[macro_export]
macro_rules! hey {
    ($($arg:tt)*) => { $crate::_hey(format_args!($($arg)*)) }
}

pub fn _nay(args: fmt::Arguments) {
    raw_log(format!("{}✘", Color::Red), Color::BrightRed, args);
}

#[macro_export]
macro_rules! nay {
    ($($arg:tt)*) => { $crate::_nay(format_args!($($arg)*)) }
}

mod tests {

    #[test]
    fn test_change_serialization() {
        let change_type = crate::change::ChangeType::Insert { new: "I just typed this instantaniously!".to_string() };
        let change = crate::change::ChangeFile {
            file_path: "cool_file.txt".to_string(),
            change_type,
            loc: crate::change::Location { line: 10, chr: 30 }
        };

        let bytes = change.to_segments();

        let deser = crate::change::ChangeFile::from_segments(bytes).expect("Failed to parse from segments!");

        println!("{}", deser)
    }

}