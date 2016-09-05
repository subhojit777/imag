/// This function prints the string `s` via the `warn!()` macro and then exits with the code `code`
/// as status.
///
/// Convenience function to be used in matches to remove one scope:
///
/// ```ignore
/// use libimagutil::warn_exit::warn_exit;
///
/// let r: Result<i32, i32> = Err(1);
/// match r {
///     Err(e) => warn_exit("Warning!", 42),
///     Ok(num) => { /* ... */ }
/// }
/// ```
///
pub fn warn_exit(s: &str, code: i32) -> ! {
    use std::process::exit;

    warn!("{}", s);
    exit(code);
}

