//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::error::Error;
use std::io::Write;
use std::io::stderr;

use ansi_term::Colour::Red;

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
    write!(stderr(), "\n").ok();
}

/// Convenience function: calls `trace_error()` with `e` and afterwards `std::process::exit()`
/// with `code`
pub fn trace_error_exit(e: &Error, code: i32) -> ! {
    use std::process::exit;

    debug!("Tracing error...");
    trace_error(e);
    debug!("Calling exit()");
    exit(code);
}

/// Print an Error type and its cause recursively, but only `max` levels
///
/// Output is the same as for `trace_error()`, though there are only `max` levels printed.
pub fn trace_error_maxdepth(e: &Error, max: u64) {
    let n = count_error_causes(e);
    let msg = Red.blink().paint(format!("{}/{} Levels of errors will be printed\n",
                                        (if max > n { n } else { max }), n));
    write!(stderr(), "{}", msg).ok();
    print_trace_maxdepth(n, e, max);
    write!(stderr(), "").ok();
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
        e.cause().map(|cause| {
            match print_trace_maxdepth(idx - 1, cause, max) {
                None    => write!(stderr(), "\n").ok(),
                Some(_) => write!(stderr(), " -- caused:\n").ok(),
            };
        });
    } else {
        write!(stderr(), "\n").ok();
    }
    write!(stderr(), "{}: {}", Red.paint(format!("ERROR[{:>4}]", idx)), e.description()).ok();
    e.cause()
}

/// Count errors in `Error::cause()` recursively
fn count_error_causes(e: &Error) -> u64 {
    1 + e.cause().map(|c| count_error_causes(c)).unwrap_or(0)
}

fn print_trace_dbg(idx: u64, e: &Error) {
    debug!("{}: {}", Red.blink().paint(format!("ERROR[{:>4}]", idx)), e.description());
    if e.cause().is_some() {
        e.cause().map(|c| print_trace_dbg(idx + 1, c));
    }
}

/// Helper functions for `Result<T, E>` types to reduce overhead in the following situations:
///
/// ```ignore
/// function().map_err(|e| { trace_error(&e); e })
/// ```
///
/// and variants
pub trait MapErrTrace {
    fn map_err_trace(self) -> Self;
    fn map_err_dbg_trace(self) -> Self;
    fn map_err_trace_exit(self, code: i32) -> Self;
    fn map_err_trace_maxdepth(self, max: u64) -> Self;
}

impl<U, E: Error> MapErrTrace for Result<U, E> {

    /// Simply call `trace_error()` on the Err (if there is one) and return the error.
    ///
    /// This does nothing besides the side effect of printing the error trace
    fn map_err_trace(self) -> Self {
        self.map_err(|e| { trace_error(&e); e })
    }

    /// Simply call `trace_error_dbg()` on the Err (if there is one) and return the error.
    ///
    /// This does nothing besides the side effect of printing the error trace
    fn map_err_dbg_trace(self) -> Self {
        self.map_err(|e| { trace_error_dbg(&e); e })
    }

    /// Simply call `trace_error_exit(code)` on the Err (if there is one).
    ///
    /// This does not return if there is an Err(e).
    fn map_err_trace_exit(self, code: i32) -> Self {
        self.map_err(|e| { trace_error_exit(&e, code) })
    }

    /// Simply call `trace_error_maxdepth(max)` on the Err (if there is one) and return the error.
    ///
    /// This does nothing besides the side effect of printing the error trace to a certain depth
    fn map_err_trace_maxdepth(self, max: u64) -> Self {
        self.map_err(|e| { trace_error_maxdepth(&e, max); e })
    }

}

