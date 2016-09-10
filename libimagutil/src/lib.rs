#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate regex;
extern crate tempfile;

#[macro_use] mod log_result;
pub mod debug_result;
pub mod edit;
pub mod info_result;
pub mod ismatch;
pub mod iter;
pub mod key_value_split;
pub mod variants;
pub mod warn_exit;
pub mod warn_result;
