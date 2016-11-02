extern crate clap;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagutil;
#[macro_use] extern crate version;

use clap::Shell;
use libimagrt::runtime::Runtime;

macro_rules! gen_types_buildui {
    ($(($p:expr, $n:ident)$(,)*)*) => (
        $(
            mod $n {
                include!($p);
            }
         )*
        )
}

gen_types_buildui!(
    ("../imag-bookmark/src/ui.rs",  imagbookmark),
    ("../imag-counter/src/ui.rs",   imagcounter),
    ("../imag-diary/src/ui.rs",     imagdiary),
    ("../imag-link/src/ui.rs",      imaglink),
    ("../imag-notes/src/ui.rs",     imagnotes),
    ("../imag-ref/src/ui.rs",       imagref),
    ("../imag-store/src/ui.rs",     imagstore),
    ("../imag-tag/src/ui.rs",       imagtag),
    ("../imag-todo/src/ui.rs",      imagtodo),
    ("../imag-view/src/ui.rs",      imagview)
    );

fn main() {
    let mut app = Runtime::get_default_cli_builder(
        "imag",
        &version!()[..],
        "imag")
        .subcommand(
            imagbookmark::build_ui(Runtime::get_default_cli_builder(
                    "bookmark",
                    &version!()[..],
                    "bookmark")))
        .subcommand(
            imagcounter::build_ui(Runtime::get_default_cli_builder(
                    "counter",
                    &version!()[..],
                    "counter")))
        .subcommand(
            imagdiary::build_ui(Runtime::get_default_cli_builder(
                    "diary",
                    &version!()[..],
                    "diary")))
        .subcommand(
            imaglink::build_ui(Runtime::get_default_cli_builder(
                    "link",
                    &version!()[..],
                    "link")))
        .subcommand(
            imagnotes::build_ui(Runtime::get_default_cli_builder(
                    "notes",
                    &version!()[..],
                    "notes")))
        .subcommand(
            imagref::build_ui(Runtime::get_default_cli_builder(
                    "ref",
                    &version!()[..],
                    "ref")))
        .subcommand(
            imagstore::build_ui(Runtime::get_default_cli_builder(
                    "store",
                    &version!()[..],
                    "store")))
        .subcommand(
            imagtag::build_ui(Runtime::get_default_cli_builder(
                    "tag",
                    &version!()[..],
                    "tag")))
        .subcommand(
            imagtodo::build_ui(Runtime::get_default_cli_builder(
                    "todo",
                    &version!()[..],
                    "todo")))
        .subcommand(
            imagview::build_ui(Runtime::get_default_cli_builder(
                    "view",
                    &version!()[..],
                    "view")))
        ;
    app.gen_completions("imag", Shell::Bash, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Fish, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Zsh, env!("OUT_DIR"));

}
