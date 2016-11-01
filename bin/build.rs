extern crate clap;
extern crate libimagrt;
#[macro_use] extern crate version;

use clap::Shell;
use libimagrt::runtime::Runtime;

include!("../imag-store/src/ui.rs");

macro_rules! gen_types_buildui {
    ($(($p:expr, $n:ident)$(,)*)*) => (
        trait _buildui_fn_type_trait {
            fn build_ui<'a>(app : App<'a, 'a>) -> App<'a, 'a>;
        }
        $(
            struct $n;
            impl $n {
                pub fn new() -> Self {
                    {}
                }
            }
            impl _buildui_fn_type_trait for $n {
                include!($p);
            }
         )*
        )
}

gen_types_buildui!(("../imag-store/src/ui.rs", imagstore), ("../imag-todo/src/ui.rs", imagtodo));

fn main() {
    let mut app = Runtime::get_default_cli_builder(
        "imag",
        &version!()[..],
        "imag foo bar");
    let v = vec![("store", imagstore::new()), ("todo", imagtodo::new())];
    for (name, obj) in v {
        app
            .subcommand(
                obj::build_ui(Runtime::get_default_cli_builder(
                        name,
                        &version!()[..],
                        name)));
    }
    app.gen_completions("imag", Shell::Bash, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Fish, env!("OUT_DIR"));
    app.gen_completions("imag", Shell::Zsh, env!("OUT_DIR"));

}
