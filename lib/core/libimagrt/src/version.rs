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

#[macro_export]
macro_rules! make_imag_version {
    () => {{
        let pkg_version = option_env!("CARGO_PKG_VERSION");
        let git_version = option_env!("CARGO_BUILD_VERSION");

        match (git_version, pkg_version) {
            (Some(git_version), Some(pkg_version)) => if git_version == "" {
                String::from(pkg_version)
            } else {
                String::from(git_version)
            },

            // imag is not beeing build with cargo... we have to set it by hand here.
            _ => String::from("0.8.0"),
        }
    }}
}
