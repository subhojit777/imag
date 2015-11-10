use runtime::Runtime;
use module::Module;
use module::ModuleResult;
use module::ModuleError;
use std::path::Path;
use std::result::Result;
use clap::ArgMatches;
use regex::Regex;

mod header;

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

    fn execute(&self, rt : &Runtime) -> ModuleResult {
        let cmd = rt.config.cli_matches.subcommand_matches("bm").unwrap();
        match cmd.subcommand_name() {
            Some("add")     => {
                debug!("Calling 'add'...");
                add(rt, cmd.subcommand_matches("add").unwrap())
            }
            Some("list")    => {
                debug!("Calling 'list'...");
                list(rt, cmd.subcommand_matches("list").unwrap())
            }
            Some("remove")  => {
                debug!("Calling 'remove'...");
                list(rt, cmd.subcommand_matches("remove").unwrap())
            }
            _ => {
                info!("Not calling any of add, list, remove");
                Ok(())
            }
        }
    }

    fn shutdown(&self, rt : &Runtime) -> ModuleResult {
        Ok(())
    }
}

fn add<'a>(rt: &Runtime, sub: &ArgMatches<'a, 'a>) -> ModuleResult {
    let url = sub.value_of("url").unwrap();
    let tags = get_tags(rt, sub);
    info!("Adding url '{}' with tags '{:?}'", url, tags);

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

