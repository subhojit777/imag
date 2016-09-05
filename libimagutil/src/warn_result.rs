// Generates a extension for the `Result<T, E>`, named `DebugResult` which has functionality to
// print either `T` or `E` via `warn!()`.
generate_result_logging_extension!(
    WarnResult,
    map_warn,
    map_warn_str,
    map_warn_err,
    map_warn_err_str,
    |s| { warn!("{}", s); }
);

