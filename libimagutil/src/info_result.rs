// Generates a extension for the `Result<T, E>`, named `DebugResult` which has functionality to
// print either `T` or `E` via `info!()`.
generate_result_logging_extension!(
    InfoResult,
    map_info,
    map_info_str,
    map_info_err,
    map_info_err_str,
    |s| { info!("{}", s); }
);
