generate_result_logging_extension!(
    DebugResult,
    map_dbg,
    map_dbg_str,
    map_dbg_err,
    map_dbg_err_str,
    |s| { debug!("{}", s); }
);
