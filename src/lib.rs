use std::{fmt::Display, process::exit};

/// See [`fatal`].
pub fn fatal_hook(msg: impl Display) -> ! {
    #[cfg(feature = "colored")]
    use colored::Colorize;
    #[cfg(feature = "colored")]
    let prefix = "fatal:".bold().red();
    #[cfg(not(feature = "colored"))]
    let prefix = "fatal:";
    eprintln!("{prefix} {msg}");
    exit(-1)
}

/// Print an error message and stop the program.
///
/// This would use [`Display`] to format the message and add a "fatal:" prefix.
/// If the `colored` feature is enabled, the prefix would be colored in red and bold using ANSI escape codes.
///
/// Process is immediately terminated with exit code -1.
///
/// # Examples
///
/// ```rust
/// use stop::fatal;
///
/// fn display() -> ! {
///     fatal!(42) // fatal: 42
/// }
///
/// fn format() -> ! {
///     fatal!("this is {}", "an error") // fatal: this is an error
/// }
///
/// fn result(result: Result<(), Box<dyn std::error::Error>>) {
///     result.unwrap_or_else(fatal!()); // `fatal!()` returns a function
/// }
///
/// ```
#[macro_export]
macro_rules! fatal {
    () => {
        |x| fatal!(x)
    };
    ($err:expr) => {
        {
            #[allow(unreachable_code)]
            {
                let err = $err;
                $crate::fatal_hook(format!("{err}"));
            };
        }
    };
    ($($arg:tt)*) => {
        {
            #[allow(unreachable_code)]
            {
                $crate::fatal_hook(format_args!($($arg)*));
            };
        }
    };
}
