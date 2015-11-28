use runtime::Runtime;
use storage::backend::StorageBackend;

use module::Module;
use module::CommandResult;
use module::CommandEnv;

use module::bm::header::build_header;
use storage::json::parser::JsonHeaderParser;
use storage::parser::FileHeaderParser;

use clap::ArgMatches;
use regex::Regex;

pub fn add_command(module: &Module, env: CommandEnv) -> CommandResult {
    let url = env.matches.value_of("url").unwrap();
    let tags = get_tags(env.rt, env.matches);
    info!("Adding url '{}' with tags '{:?}'", url, tags);

    let header = build_header(&String::from(url), &tags);
    let jheader = JsonHeaderParser::new(None).write(&header);
    println!("JSON: {:?}", jheader);

    Ok(())
}

pub fn list_command(module: &Module, env: CommandEnv) -> CommandResult {
    let tags    = get_tags(env.rt, env.matches);
    let matcher = get_matcher(env.rt, env.matches);

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

pub fn remove_command(module: &Module, env: CommandEnv) -> CommandResult {
    let tags    = get_tags(env.rt, env.matches);
    let matcher = get_matcher(env.rt, env.matches);
    let id      = get_id(env.rt, env.matches);

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

/*
 *
 * Private helpers
 *
 */

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

