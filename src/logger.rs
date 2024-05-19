//! To do logging

use color_print::cprintln;

#[allow(dead_code)]
pub(crate) fn patch_info(args: std::fmt::Arguments) {
    let info = format!("{}", args);
    cprintln!("<green><bold>{}</bold></green> {}", "[INFO]", info);
}

#[allow(dead_code)]
pub(crate) fn patch_error(args: std::fmt::Arguments) {
    let error = format!("{}", args);
    cprintln!("<red><bold>{}</bold></red> {}", "[ERROR]", error);
}

#[allow(dead_code)]
pub(crate) fn patch_warn(args: std::fmt::Arguments) {
    let warn = format!("{}", args);
    cprintln!("<yellow><bold>{}</bold></yellow> {}", "[WARN]", warn);
}

#[allow(unused)]
macro_rules! info_log{
    ($($arg:tt)*) => {
        crate::logger::patch_info(format_args!($($arg)*));
    };
}

macro_rules! error_log {
    ($($arg:tt)*) => {
        crate::logger::patch_error(format_args!($($arg)*));
    };
}

macro_rules! warn_log {
    ($($arg:tt)*) => {
        crate::logger::patch_warn(format_args!($($arg)*));
    };
}
