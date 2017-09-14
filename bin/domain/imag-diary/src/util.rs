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

use libimagrt::runtime::Runtime;
use libimagdiary::error::*;

use toml::Value;
use toml_query::read::TomlValueReadExt;

pub fn get_diary_name(rt: &Runtime) -> Option<String> {
    use libimagdiary::config::get_default_diary_name;

    get_default_diary_name(rt)
        .or(rt.cli().value_of("diaryname").map(String::from))
}

pub enum Timed {
    Hourly,
    Minutely,
}

/// Returns true if the diary should always create timed entries, which is whenever
///
/// ```toml
/// diary.diaries.<diary>.timed = true
/// ```
///
/// # Returns
///
/// * Ok(Some(Timed::Hourly)) if diary should create timed entries
/// * Ok(Some(Timed::Minutely)) if diary should not create timed entries
/// * Ok(None) if config is not available
/// * Err(e) if reading the toml failed, type error or something like this
///
pub fn get_diary_timed_config(rt: &Runtime, diary_name: &str) -> Result<Option<Timed>> {
    match rt.config() {
        None      => Ok(None),
        Some(cfg) => {
            let v = cfg
                .config()
                .read(&format!("diary.diaries.{}.timed", diary_name))
                .chain_err(|| DiaryErrorKind::IOError);

            match v {
                Ok(Some(&Value::String(ref s))) => parse_timed_string(s, diary_name).map(Some),

                Ok(Some(_)) => {
                    let s = format!("Type error at 'diary.diaryies.{}.timed': should be either 'h'/'hourly' or 'm'/'minutely'", diary_name);
                    Err(s).map_err(From::from)
                },

                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        }
    }
}

pub fn parse_timed_string(s: &str, diary_name: &str) -> Result<Timed> {
    if s == "h" || s == "hourly" {
        Ok(Timed::Hourly)
    } else if s == "m" || s == "minutely" {
        Ok(Timed::Minutely)
    } else {
        let s = format!("Cannot parse config: 'diary.diaries.{}.timed = {}'",
                        diary_name, s);
        Err(s).map_err(From::from)
    }
}
