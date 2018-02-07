//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

/// Generator helper macro for mock app (for testing only)
///
/// Requires the following crates in scope:
///
/// * std
/// * libimagrt
/// * clap
///
#[macro_export]
macro_rules! make_mock_app {
    {
        app $appname:expr;
        module $module:ident;
        version $version:expr;
    } => {
        make_mock_app! {
            app $appname;
            module $module;
            version $version;
            with help "This is a mocking app";
        }
    };

    {
        app $appname:expr;
        modulename $module:ident;
        version $version:expr;
        with help $help:expr;
    }=> {
        mod $module {
            use clap::{App, ArgMatches};
            use libimagrt::spec::CliSpec;
            use libimagrt::runtime::Runtime;
            use libimagrt::error::RuntimeError;
            use libimagrt::configuration::InternalConfiguration;
            use toml::Value;

            #[derive(Clone)]
            struct MockLinkApp<'a> {
                args: Vec<&'static str>,
                inner: App<'a, 'a>,
            }

            impl<'a> MockLinkApp<'a> {
                fn new(args: Vec<&'static str>) -> Self {
                    MockLinkApp {
                        args: args,
                        inner: ::build_ui(Runtime::get_default_cli_builder($appname, $version, $help)),
                    }
                }
            }

            impl<'a> CliSpec<'a> for MockLinkApp<'a> {
                fn name(&self) -> &str {
                    self.inner.get_name()
                }

                fn matches(self) -> ArgMatches<'a> {
                    self.inner.get_matches_from(self.args)
                }
            }

            impl<'a> InternalConfiguration for MockLinkApp<'a> {
                fn enable_logging(&self) -> bool {
                    false
                }

                fn use_inmemory_fs(&self) -> bool {
                    true
                }
            }

            #[allow(unused)]
            pub fn generate_minimal_test_config() -> Option<Value> {
                ::toml::de::from_str("[store]\nimplicit-create=true").ok()
            }

            #[allow(unused)]
            pub fn generate_test_runtime<'a>(mut args: Vec<&'static str>) -> Result<Runtime<'a>, RuntimeError> {
                let mut cli_args = vec![$appname, "--rtp", "/tmp"];

                cli_args.append(&mut args);

                let cli_app = MockLinkApp::new(cli_args);
                Runtime::with_configuration(cli_app, generate_minimal_test_config())
            }

            #[allow(unused)]
            pub fn reset_test_runtime<'a>(mut args: Vec<&'static str>, old_runtime: Runtime)
                -> Result<Runtime<'a>, RuntimeError>
            {
                let mut cli_args = vec![$appname, "--rtp", "/tmp"];

                cli_args.append(&mut args);

                let cli_app = MockLinkApp::new(cli_args);
                Runtime::with_configuration(cli_app, generate_minimal_test_config())
                    .map(|rt| rt.with_store(old_runtime.extract_store()))
            }
        }
    };

}

