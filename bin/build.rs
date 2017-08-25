//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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
    ("../../../imag-link/src/ui.rs",      imaglink),
    ("../../../imag-notes/src/ui.rs",     imagnotes),
    ("../../../imag-ref/src/ui.rs",       imagref),
    ("../../../imag-store/src/ui.rs",     imagstore),
    ("../../../imag-tag/src/ui.rs",       imagtag),
    ("../../../imag-view/src/ui.rs",      imagview)
);

fn main() {
    // Make the `imag`-App...
    let mut app = Runtime::get_default_cli_builder(
        "imag",
        &version!()[..],
        "imag")
        // and add all the subapps as subcommands.
        .subcommand(build_subcommand!("link",       imaglink))
        .subcommand(build_subcommand!("notes",      imagnotes))
        .subcommand(build_subcommand!("ref",        imagref))
        .subcommand(build_subcommand!("store",      imagstore))
        .subcommand(build_subcommand!("tag",        imagtag))
        .subcommand(build_subcommand!("view",       imagview));

    let outdir = std::env::var("OUT_DIR").unwrap();

    // Actually generates the completion files
    app.gen_completions("imag", Shell::Bash, outdir.clone());
    app.gen_completions("imag", Shell::Fish, outdir.clone());
    app.gen_completions("imag", Shell::Zsh,  outdir);

}

