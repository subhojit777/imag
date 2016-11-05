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

macro_rules! build_subcommand {
    ($name:expr, $module:ident) => (
        $module::build_ui(Runtime::get_default_cli_builder(
                $name,
                &version!()[..],
                $name))
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
        .subcommand(build_subcommand!("bookmark",   imagbookmark))
        .subcommand(build_subcommand!("counter",    imagcounter))
        .subcommand(build_subcommand!("diary",      imagdiary))
        .subcommand(build_subcommand!("link",       imaglink))
        .subcommand(build_subcommand!("notes",      imagnotes))
        .subcommand(build_subcommand!("ref",        imagref))
        .subcommand(build_subcommand!("store",      imagstore))
        .subcommand(build_subcommand!("tag",        imagtag))
        .subcommand(build_subcommand!("todo",       imagtodo))
        .subcommand(build_subcommand!("view",       imagview));

    app.gen_completions("imag", Shell::Bash, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Fish, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Zsh, env!("OUT_DIR"));

}

