extern crate clap;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagutil;
#[macro_use] extern crate version;

use clap::Shell;
use libimagrt::runtime::Runtime;

mod toplevelbuildscript {
    include!("../../../build.rs");
    pub fn build() {
        self::main();
    }
}

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
        $module::build_ui(Runtime::get_default_cli_builder($name, &version!()[..], $name))
    )
}

// Actually generates the module.
gen_mods_buildui!(
    ("../../../bin/core/imag-annotate/src/ui.rs",    imagannotate),
    ("../../../bin/core/imag-diagnostics/src/ui.rs", imagdiagnostics),
    ("../../../bin/core/imag-edit/src/ui.rs",        imagedit),
    ("../../../bin/core/imag-gps/src/ui.rs",         imaggps),
    ("../../../bin/core/imag-grep/src/ui.rs",        imaggrep),
    ("../../../bin/core/imag-ids/src/ui.rs",         imagids),
    ("../../../bin/core/imag-init/src/ui.rs",        imaginit),
    ("../../../bin/core/imag-link/src/ui.rs",        imaglink),
    ("../../../bin/core/imag-mv/src/ui.rs",          imagmv),
    ("../../../bin/core/imag-ref/src/ui.rs",         imagref),
    ("../../../bin/core/imag-store/src/ui.rs",       imagstore),
    ("../../../bin/core/imag-tag/src/ui.rs",         imagtag),
    ("../../../bin/core/imag-view/src/ui.rs",        imagview)
    ("../../../bin/domain/imag-bookmark/src/ui.rs",  imagbookmark),
    ("../../../bin/domain/imag-contact/src/ui.rs",   imagcontact),
    ("../../../bin/domain/imag-diary/src/ui.rs",     imagdiary),
    ("../../../bin/domain/imag-habit/src/ui.rs",     imaghabit),
    ("../../../bin/domain/imag-log/src/ui.rs",       imaglog),
    ("../../../bin/domain/imag-mail/src/ui.rs",      imagmail),
    ("../../../bin/domain/imag-notes/src/ui.rs",     imagnotes),
    ("../../../bin/domain/imag-timetrack/src/ui.rs", imagtimetrack),
    ("../../../bin/domain/imag-todo/src/ui.rs",      imagtodo),
);

fn main() {
    // Make the `imag`-App...
    let mut app = Runtime::get_default_cli_builder(
        "imag",
        &version!()[..],
        "imag")
        // and add all the subapps as subcommands.
        .subcommand(build_ui!("annotate",    imagannotate))
        .subcommand(build_ui!("diagnostics", imagdiagnostics))
        .subcommand(build_ui!("edit",        imagedit))
        .subcommand(build_ui!("gps",         imaggps))
        .subcommand(build_ui!("grep",        imaggrep))
        .subcommand(build_ui!("ids",         imagids))
        .subcommand(build_ui!("init",        imaginit))
        .subcommand(build_ui!("link",        imaglink))
        .subcommand(build_ui!("mv",          imagmv))
        .subcommand(build_ui!("ref",         imagref))
        .subcommand(build_ui!("store",       imagstore))
        .subcommand(build_ui!("tag",         imagtag))
        .subcommand(build_ui!("view",        imagview))
        .subcommand(build_ui!("bookmark",    imagbookmark))
        .subcommand(build_ui!("contact",     imagcontact))
        .subcommand(build_ui!("diary",       imagdiary))
        .subcommand(build_ui!("habit",       imaghabit))
        .subcommand(build_ui!("log",         imaglog))
        .subcommand(build_ui!("mail",        imagmail))
        .subcommand(build_ui!("notes",       imagnotes))
        .subcommand(build_ui!("timetrack",   imagtimetrack))
        .subcommand(build_ui!("todo",        imagtodo));

    // Actually generates the completion files
    app.gen_completions("imag", Shell::Bash, "../../../target/shell-completions.d/");
    app.gen_completions("imag", Shell::Fish, "../../../target/shell-completions.d/");
    app.gen_completions("imag", Shell::Zsh,  "../../../target/shell-completions.d/");

    toplevelbuildscript::build();
}

