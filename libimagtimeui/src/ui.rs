pub fn time_ui_fmtstr() -> &'static str {
    "YYYY-MM-DD[THH[:mm[:ss]]]"
}

pub fn time_ui_fmtstr_expl() -> &'static str {
    #r"In the UI, the format for Time is always YEAR-MONTH-DAY.
    Optionally, Time can be specified by seperating it from the date with 'T'.
    Minutes and Seconds are optional.
    "#
}


