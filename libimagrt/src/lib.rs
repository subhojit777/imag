#[macro_use] extern crate log;
#[macro_use] extern crate itertools;
#[cfg(unix)] extern crate xdg_basedir;

extern crate clap;
extern crate config;

extern crate libimagstore;
extern crate libimagutil;

mod configuration;
mod logger;
mod runtime;

