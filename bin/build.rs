extern crate clap;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagutil;
#[macro_use] extern crate version;

use clap::Shell;
use libimagrt::runtime::Runtime;

/// This macro generates mods with the given '$modulename',
/// whose content is the file given with `$path`.
/// In this case, It is used specifically to include the
/// `ui.rs` files of the imag binaries.
/// The imag project (accidentally?) followed the convention
/// to write a `ui.rs` containing the function
/// `fn build_ui(app : App) -> App`.
/// This macro allows us to use the same named functions by
/// putting them each into their own module.
macro_rules! gen_mods_buildui {
    ($(($path:expr, $modulename:ident)$(,)*)*) => (
        $(
            mod $modulename {
                include!($path);
            }
         )*
        )
}

/// This macro reduces boilerplate code.
///
/// For example: `build_subcommand!("counter", imagcounter)`
/// will result in the following code:
/// ```ignore
/// imagcounter::build_ui(Runtime::get_default_cli_builder(
///     "counter",
///     &version!()[..],
///     "counter"))
/// ```
/// As for the `&version!()[..]` part, it does not matter
/// which version the subcommand is getting here, as the
/// output of this script is a completion script, which
/// does not contain information about the version at all.
macro_rules! build_subcommand {
    ($name:expr, $module:ident) => (
        $module::build_ui(Runtime::get_default_cli_builder(
                $name,
                &version!()[..],
                $name))
    )
}

// Actually generates the module.
gen_mods_buildui!(
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
    // Make the `imag`-App...
    let mut app = Runtime::get_default_cli_builder(
        "imag",
        &version!()[..],
        "imag")
        // and add all the subapps as subcommands.
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

    // Actually generates the completion files
    app.gen_completions("imag", Shell::Bash, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Fish, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Zsh, env!("OUT_DIR"));

}

