use std::{fmt::Display, process::exit};

/// See [`stop`].
pub fn stop_impl(msg: impl Display) -> ! {
    #[cfg(feature = "colored")]
    use colored::Colorize;
    #[cfg(feature = "colored")]
    let prefix = "error".bold().red();
    #[cfg(not(feature = "colored"))]
    let prefix = "error";
    eprintln!("{prefix}: {msg}");
    exit(-1)
}

/// Print an error message and stop the program.
/// 
/// This would use [`Display`] to format the message and add an "error:" prefix.
/// 
/// Exit code is always `-1`.
///
/// # Examples
///
/// ```rust
/// use stop::stop;
///
/// fn display() -> ! {
///     stop!(42) // error: 42
/// }
///
/// fn format() -> ! {
///     stop!("this is {}", "an error") // error: this is an error
/// }
///
/// fn result(result: Result<(), Box<dyn std::error::Error>>) {
///     result.unwrap_or_else(stop!()); // `stop!()` returns a function
/// }
///
/// ```
#[macro_export]
macro_rules! stop {
    () => {
        |x| stop!(x)
    };
    ($err:expr) => {
        {
            #[allow(unreachable_code)]
            {
                let err = $err;
                $crate::stop_impl(format!("{err}"));
            };
        }
    };
    ($($arg:tt)*) => {
        {
            #[allow(unreachable_code)]
            {
                $crate::stop_impl(format_args!($($arg)*));
            };
        }
    };
}
