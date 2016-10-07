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

use clap::{Arg, ArgMatches, App, SubCommand};

use tag::Tag;

use libimagutil::cli_validators::is_tag;

/// Generates a `clap::SubCommand` to be integrated in the commandline-ui builder for building a
/// "tags --add foo --remove bar" subcommand to do tagging action.
pub fn tag_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(tag_subcommand_name())
        .author("Matthias Beyer <mail@beyermatthias.de>")
        .version("0.1")
        .about("Add or remove tags")
        .arg(tag_add_arg())
        .arg(tag_remove_arg())
}

pub fn tag_add_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(tag_subcommand_add_arg_name())
        .short("a")
        .long("add")
        .takes_value(true)
        .value_name("tags")
        .multiple(true)
        .validator(is_tag)
        .help("Add tags, seperated by comma or by specifying multiple times")
}

pub fn tag_remove_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(tag_subcommand_remove_arg_name())
        .short("r")
        .long("remove")
        .takes_value(true)
        .value_name("tags")
        .multiple(true)
        .validator(is_tag)
        .help("Remove tags, seperated by comma or by specifying multiple times")
}

pub fn tag_subcommand_name() -> &'static str {
    "tags"
}

pub fn tag_subcommand_add_arg_name() -> &'static str {
    "add-tags"
}

pub fn tag_subcommand_remove_arg_name() -> &'static str {
    "remove-tags"
}

pub fn tag_subcommand_names() -> Vec<&'static str> {
    vec![tag_subcommand_add_arg_name(), tag_subcommand_remove_arg_name()]
}

/// Generates a `clap::Arg` which can be integrated into the commandline-ui builder for building a
/// "-t" or "--tags" argument which takes values for tagging actions (add, remove)
pub fn tag_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(tag_argument_name())
        .short("t")
        .long("tags")
        .takes_value(true)
        .multiple(true)
        .validator(is_tag)
        .help("Add or remove tags, prefixed by '+' (for adding) or '-' (for removing)")
}

pub fn tag_argument_name() -> &'static str {
    "specify-tags"
}

/// Get the tags which should be added from the commandline
///
/// Returns none if the argument was not specified
pub fn get_add_tags(matches: &ArgMatches) -> Option<Vec<Tag>> {
    let add = tag_subcommand_add_arg_name();
    extract_tags(matches, add, '+')
        .or_else(|| matches.values_of(add).map(|values| values.map(String::from).collect()))
}

/// Get the tags which should be removed from the commandline
///
/// Returns none if the argument was not specified
pub fn get_remove_tags(matches: &ArgMatches) -> Option<Vec<Tag>> {
    let rem = tag_subcommand_remove_arg_name();
    extract_tags(matches, rem, '+')
        .or_else(|| matches.values_of(rem).map(|values| values.map(String::from).collect()))
}

fn extract_tags(matches: &ArgMatches, specifier: &str, specchar: char) -> Option<Vec<Tag>> {
    if let Some(submatch) = matches.subcommand_matches("tags") {
        submatch.values_of(specifier)
            .map(|values| values.map(String::from).collect())
    } else {
        matches.values_of("specify-tags")
            .map(|argmatches| {
                argmatches
                    .map(String::from)
                    .filter(|s| s.starts_with(specchar))
                    .map(|s| {
                        String::from(s.split_at(1).1)
                    })
                    .collect()
            })
    }
}

