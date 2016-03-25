use std::error::Error;
use std::io::Write;
use std::io::stderr;

/// Print an Error type and its cause recursively
///
/// The error is printed with "Error NNNN :" as prefix, where "NNNN" is a number which increases
/// which each recursion into the errors cause. The error description is used to visualize what
/// failed and if there is a cause "-- caused by:" is appended, and the cause is printed on the next
/// line.
///
/// Example output:
///
/// ```ignore
/// Error    1 : Some error -- caused by:
/// Error    2 : Some other error -- caused by:
/// Error    3 : Yet another Error -- caused by:
/// ...
///
/// Error <NNNN> : <Error description>
/// ```
pub fn trace_error(e: &Error) {
    print_trace_maxdepth(count_error_causes(e), e, ::std::u64::MAX);
    write!(stderr(), "\n");
}

/// Print an Error type and its cause recursively, but only `max` levels
///
/// Output is the same as for `trace_error()`, though there are only `max` levels printed.
pub fn trace_error_maxdepth(e: &Error, max: u64) {
    let n = count_error_causes(e);
    write!(stderr(), "{}/{} Levels of errors will be printed\n", (if max > n { n } else { max }), n);
    print_trace_maxdepth(n, e, max);
    write!(stderr(), "");
}

/// Print an Error type and its cause recursively with the debug!() macro
///
/// Output is the same as for `trace_error()`.
pub fn trace_error_dbg(e: &Error) {
    print_trace_dbg(0, e);
}

/// Helper function for `trace_error()` and `trace_error_maxdepth()`.
///
/// Returns the cause of the last processed error in the recursion, so `None` if all errors where
/// processed.
fn print_trace_maxdepth(idx: u64, e: &Error, max: u64) -> Option<&Error> {
    if e.cause().is_some() && idx > 0 {
        match print_trace_maxdepth(idx - 1, e.cause().unwrap(), max) {
            None => write!(stderr(), "\n"),
            Some(_) => write!(stderr(), " -- caused:\n"),
        };
    } else {
        write!(stderr(), "\n");
    }
    write!(stderr(), "Error {:>4} : {}", idx, e.description());
    e.cause()
}

/// Count errors in Error::cause() recursively
fn count_error_causes(e: &Error) -> u64 {
    1 + if e.cause().is_some() { count_error_causes(e.cause().unwrap()) } else { 0 }
}

fn print_trace_dbg(idx: u64, e: &Error) {
    debug!("Error {:>4} : {}", idx, e.description());
    if e.cause().is_some() {
        debug!(" -- caused by:");
        print_trace_dbg(idx + 1, e.cause().unwrap());
    }
}

