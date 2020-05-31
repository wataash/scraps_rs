// SPDX-License-Identifier: Apache-2.0

//! # scraps

// -------------------------------------------------------------------------------------------------
/// # logger

// simple colored logger

// TODO: isatty

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (crate::_log(crate::_LogLevel::Error, file!(), line!(), format_args!($($arg)*));)
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (crate::_log(crate::_LogLevel::Warn, file!(), line!(), format_args!($($arg)*));)
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (crate::_log(crate::_LogLevel::Info, file!(), line!(), format_args!($($arg)*));)
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (crate::_log(crate::_LogLevel::Debug, file!(), line!(), format_args!($($arg)*));)
}

#[doc(hidden)]
pub(crate) enum _LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

#[doc(hidden)]
pub(crate) fn _log(level: crate::_LogLevel, file: &str, line: u32, args: std::fmt::Arguments) {
    match level {
        _LogLevel::Error => eprintln!("[E] \x1b[31m{}:{} {}\x1b[0m", file, line, args),
        _LogLevel::Warn => eprintln!("[W] \x1b[33m{}:{} {}\x1b[0m", file, line, args),
        _LogLevel::Info => eprintln!("[I] \x1b[34m{}:{} {}\x1b[0m", file, line, args),
        _LogLevel::Debug => eprintln!("[D] \x1b[37m{}:{} {}\x1b[0m", file, line, args),
    }
}

/// Returns [`failure::Error`] along with logging it.
#[macro_export]
macro_rules! ret_e {
    // ref: failure-0.1.8/src/macros.rs bail!
    ($($arg:tt)*) => {
        return Err(crate::_err(file!(), line!(), format_args!($($arg)*)))
    }
}

#[doc(hidden)]
pub(crate) fn _err(file: &str, line: u32, args: std::fmt::Arguments) -> failure::Error {
    // let msg = format!("[E] \x1b[31m{}:{} {}\x1b[0m", file, line, args);
    // let err = failure::err_msg(msg);
    // TODO: issue: confusing error message:
    // error[E0277]: `core::fmt::Opaque` cannot be shared between threads safely
    //   --> src/utils.rs:51:15
    //    |
    // 51 |     let err = failure::err_msg(msg);
    //    |               ^^^^^^^^^^^^^^^^ `core::fmt::Opaque` cannot be shared between threads safely
    //    |
    //   ::: /home/wsh/.cargo/registry/src/github.com-1ecc6299db9ec823/failure-0.1.8/src/error_message.rs:11:44
    //    |
    // 11 | pub fn err_msg<D: Display + Debug + Sync + Send + 'static>(msg: D) -> Error {
    //    |                                            ---- required by this bound in `failure::error_message::err_msg`
    //    |
    //    = help: within `[std::fmt::ArgumentV1<'_>]`, the trait `std::marker::Sync` is not implemented for `core::fmt::Opaque`
    //    = note: required because it appears within the type `&core::fmt::Opaque`
    //    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
    //    = note: required because it appears within the type `[std::fmt::ArgumentV1<'_>]`
    //    = note: required because of the requirements on the impl of `std::marker::Send` for `&[std::fmt::ArgumentV1<'_>]`
    //    = note: required because it appears within the type `std::fmt::Arguments<'_>`

    let msg = format!("[E] \x1b[31m{}:{} {}\x1b[0m", file, line, args);
    let err = failure::err_msg(msg);
    eprintln!("{}", err);
    err
}

// // TODO: set logger and test
// //   rust set callback
// static mut log_func: &'static Option(fn(crate::_LogLevel, &str, u32, std::fmt::Arguments)) =
//     &None;
// pub fn set_logger(f: fn(crate::_LogLevel, &str, u32, std::fmt::Arguments)) {
//     log_func = &Some(f);
// }

#[macro_export]
macro_rules! error_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[31m[E] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}
#[macro_export]
macro_rules! warn_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[33m[W] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}
#[macro_export]
macro_rules! info_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[34m[I] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}
#[macro_export]
macro_rules! debug_simpler {
    ($($arg:tt)*) => (eprintln!("\x1b[37m[D] {}:{} {}\x1b[0m", file!(), line!(), format_args!($($arg)*)));
}

// -------------------------------------------------------------------------------------------------
/// # string

/// Returns the reference of `str` from `n`th (0-base index) to the end of `str`.
///
/// # Examples
///
/// ```
/// assert_eq!(scraps_rs::from_line_n("foo\nbar", 0), Some("foo\nbar"));
/// ```
pub fn from_line_n(s: &str, n: usize) -> Option<&str> {
    if n == 0 {
        return Some(s);
    }
    let mut m = 0;
    for (i, c) in s.char_indices() {
        if c == '\n' {
            m += 1;
            if m == n {
                return Some(&s[i + 1..]);
            }
        }
    }
    None
}

/// # Examples
///
/// ```
/// assert_eq!(scraps_rs::partial_str("abcdef", 5), "ab...");
/// ```
pub fn partial_str(s: &str, width: usize) -> String {
    if s.len() <= width {
        return s.to_string();
    }
    if width <= 3 {
        return s[..width].to_string();
    }
    format!("{}...", &s[..(width - 3)]).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_line_n() {
        use crate::from_line_n;
        ;
        assert_eq!(from_line_n("foo", 0), Some("foo"));
        assert_eq!(from_line_n("foo", 1), None);
        assert_eq!(from_line_n("foo", 2), None);
        assert_eq!(from_line_n("foo\nbar", 0), Some("foo\nbar"));
        assert_eq!(from_line_n("foo\nbar", 1), Some("bar"));
        assert_eq!(from_line_n("foo\nbar", 2), None);
        assert_eq!(from_line_n("foo\nbar\nbaz", 0), Some("foo\nbar\nbaz"));
        assert_eq!(from_line_n("foo\nbar\nbaz", 1), Some("bar\nbaz"));
        assert_eq!(from_line_n("foo\nbar\nbaz", 2), Some("baz"));
        assert_eq!(from_line_n("\n\n\n", 0), Some("\n\n\n"));
        assert_eq!(from_line_n("\n\n\n", 1), Some("\n\n"));
        assert_eq!(from_line_n("\n\n\n", 2), Some("\n"));
        assert_eq!(from_line_n("\n\n\n", 3), Some(""));
        assert_eq!(from_line_n("\n\n\n", 4), None);
        assert_eq!(
            from_line_n("\nこんにちは\n世界\n", 0),
            Some("\nこんにちは\n世界\n")
        );
        assert_eq!(
            from_line_n("\nこんにちは\n世界\n", 1),
            Some("こんにちは\n世界\n")
        );
        assert_eq!(from_line_n("\nこんにちは\n世界\n", 2), Some("世界\n"));
        assert_eq!(from_line_n("\nこんにちは\n世界\n", 3), Some(""));
        assert_eq!(from_line_n("\nこんにちは\n世界\n", 4), None);
    }

    #[test]
    fn partial_str() {
        use crate::partial_str;
        partial_str("", 0);
        assert_eq!(partial_str("", 0), "");
        assert_eq!(partial_str("a", 0), "");
        assert_eq!(partial_str("a", 1), "a");
        assert_eq!(partial_str("ab", 1), "a");
        assert_eq!(partial_str("ab", 2), "ab");
        assert_eq!(partial_str("abc", 2), "ab");
        assert_eq!(partial_str("abc", 3), "abc");
        assert_eq!(partial_str("abcd", 3), "abc");
        assert_eq!(partial_str("abcd", 4), "abcd");
        assert_eq!(partial_str("abcde", 4), "a...");
        assert_eq!(partial_str("abcde", 5), "abcde");
        assert_eq!(partial_str("abcdef", 5), "ab...");
    }
}
