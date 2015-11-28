use runtime::Runtime;
use module::Module;
use module::ModuleResult;
use module::ModuleError;
use std::path::Path;
use std::result::Result;
use clap::ArgMatches;
use regex::Regex;

mod header;
mod commands;

use self::header::build_header;
use storage::json::parser::JsonHeaderParser;
use storage::parser::FileHeaderParser;

use self::commands::add::*;
use self::commands::list::*;
use self::commands::remove::*;

pub struct BMModule {
    path: Option<String>,
}

const CALLNAMES : &'static [&'static str] = &[ "bm", "bookmark" ];

impl Module for BMModule {

    fn new(rt : &Runtime) -> BMModule {
        BMModule {
            path: None
        }
    }

    fn callnames() -> &'static [&'static str] {
        CALLNAMES
    }

    fn name(&self) -> &'static str{
        "Bookmark"
    }

    fn shutdown(&self, rt : &Runtime) -> ModuleResult {
        Ok(())
    }

    fn get_commands<EC: ExecutableCommand>(&self, rt: &Runtime) -> Vec<EC> {
        vec![
            AddCommand::new(),
            ListCommand::new(),
            RemoveCommand::new(),
        ]
    }
}

fn add<'a>(rt: &Runtime, sub: &ArgMatches<'a, 'a>) -> ModuleResult {
    let url = sub.value_of("url").unwrap();
    let tags = get_tags(rt, sub);
    info!("Adding url '{}' with tags '{:?}'", url, tags);

    let header = build_header(&String::from(url), &tags);
    let jheader = JsonHeaderParser::new(None).write(&header);
    println!("JSON: {:?}", jheader);

    Ok(())
}

fn list<'a>(rt: &Runtime, sub: &ArgMatches<'a, 'a>) -> ModuleResult {
    let tags    = get_tags(rt, sub);
    let matcher = get_matcher(rt, sub);

    match matcher {
        Some(reg) => {
            info!("Listing urls with matcher '{}' and with tags {:?}",
                     reg.as_str(),
                     tags);
        }
        None => {
            info!("Listing urls with tags {:?}", tags);
        }
    }

    Ok(())
}

fn remove<'a>(rt: &Runtime, sub: &ArgMatches<'a, 'a>) -> ModuleResult {
    let tags    = get_tags(rt, sub);
    let matcher = get_matcher(rt, sub);
    let id      = get_id(rt, sub);

    match id {
        Some(idstr) => {
            info!("Removing urls with id '{}'", idstr);
        }
        None => {
            match matcher {
                Some(reg) => {
                    info!("Removing urls with matcher '{}' and with tags {:?}",
                             reg.as_str(), tags);
                }
                None => {
                    info!("Listing urls with tags {:?}", tags);
                }
            }
        }
    }

    Ok(())
}

fn get_tags<'a>(rt: &Runtime, sub: &ArgMatches<'a, 'a>) -> Vec<String> {
    debug!("Fetching tags from commandline");
    sub.value_of("tags").and_then(|tags|
                                  Some(tags.split(",")
                                       .into_iter()
                                       .map(|s| s.to_string())
                                       .filter(|e|
                                            if e.contains(" ") {
                                                warn!("Tag contains spaces: '{}'", e);
                                                false
                                            } else {
                                                true
                                            }).collect()
                                      )
                                 ).or(Some(vec![])).unwrap()

}

fn get_matcher<'a>(rt: &Runtime, sub: &ArgMatches<'a, 'a>) -> Option<Regex> {
    debug!("Fetching matcher from commandline");
    if let Some(s) = sub.value_of("match") {
        if let Ok(r) = Regex::new(s) {
            return Some(r)
        } else {
            error!("Regex error, continuing without regex");
        }
    }
    None

}

fn get_id<'a>(rt: &Runtime, sub: &ArgMatches<'a, 'a>) -> Option<String> {
    debug!("Fetching id from commandline");
    sub.value_of("id").and_then(|s| Some(String::from(s)))
}

