//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 the imag contributors
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

use std::process::Command;
fn main() {
    let profile = String::from(std::env::var("PROFILE").unwrap());
    let git_hash = if profile == "debug" {
        let output = Command::new("git")
            .args(&["rev-parse", "--short=10", "HEAD"])
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    } else {
        String::from("")
    };

    println!("cargo:rustc-env=CARGO_BUILD_GIT_HASH={}", git_hash);
}
