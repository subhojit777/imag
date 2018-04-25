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

use std::path::Path;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

use error::Result;

pub fn read_to_string<A: AsRef<Path> + Debug>(pb: A) -> Result<String> {
    let mut cont = String::new();

    let mut file = File::open(pb.as_ref())?;
    let bytes = file.read_to_string(&mut cont)?;

    debug!("Read {} bytes from {:?}", bytes, pb);

    Ok(cont)
}

