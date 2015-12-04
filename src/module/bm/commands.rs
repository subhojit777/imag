use runtime::Runtime;
use storage::backend::{StorageBackendError, StorageBackend};

use module::Module;
use module::ModuleError;
use module::CommandResult;
use module::CommandEnv;

use module::bm::header::build_header;
use module::bm::header::get_tags_from_header;
use storage::json::parser::JsonHeaderParser;
use storage::parser::{Parser, FileHeaderParser};
use storage::file::File;
use ui::file::{FilePrinter, TablePrinter};
use std::vec::IntoIter;

use clap::ArgMatches;
use regex::Regex;

pub fn add_command(module: &Module, env: CommandEnv) -> CommandResult {
    let url = env.matches.value_of("url").unwrap();
    let tags = get_tags(env.rt, env.matches);
    info!("Adding url '{}' with tags '{:?}'", url, tags);

    let header  = build_header(&String::from(url), &tags);
    let file    = File::new_with_header(module, header);
    let parser  = Parser::new(JsonHeaderParser::new(None));
    let putres  = env.bk.put_file(file, &parser);

    putres.map_err(|sberr| {
        let mut err = ModuleError::new("Storage Backend Error");
        err.caused_by = Some(Box::new(sberr));
        err
    })
}

pub fn list_command(module: &Module, env: CommandEnv) -> CommandResult {
    let printer = TablePrinter::new(env.rt.is_verbose(), env.rt.is_debugging());
    let files   = get_filtered_files_from_backend(module, &env);

    debug!("Printing files now");
    files.map(|f| printer.print_files(f));

    Ok(())
}

pub fn remove_command(module: &Module, env: CommandEnv) -> CommandResult {
    let checked : bool = run_removal_checking(&env);
    debug!("Checked mode: {}", checked);
    if let Some(id) = get_id(env.rt, env.matches) {
        debug!("Remove by id: {}", id);

        let parser = Parser::new(JsonHeaderParser::new(None));
        let file   = env.bk.get_file_by_id(module, &id.into(), &parser).unwrap();
        debug!("Remove file  : {:?}", file);

        if let Err(e) = env.bk.remove_file(module, file, checked) {
            debug!("Remove failed");
            let mut err = ModuleError::new("Removing file failed");
            err.caused_by = Some(Box::new(e));
            Err(err)
        } else {
            debug!("Remove worked");
            Ok(())
        }
    } else {
        debug!("Remove more than one file");

        let files = get_filtered_files_from_backend(module, &env);
        let nfiles = files.len();
        info!("Removing {} Files", nfiles);

        let errs = files.map(|file| {
                debug!("Remove file: {:?}", file);
                env.bk.remove_file(module, file, checked)
            })
            .filter(|e| e.is_err())
            .map(|e| {
                let err = e.err().unwrap();
                warn!("Error occured in Filesystem operation: {}", err);
                err
            })
            .collect::<Vec<StorageBackendError>>();

        let nerrs = errs.len();

        if nerrs != 0 {
            warn!("{} Errors occured while removing {} files", nerrs, nfiles);
            let moderr = ModuleError::new("File removal failed");

            // TODO : Collect StorageBackendErrors

            Err(moderr)
        } else {
            Ok(())
        }
    }
}

/*
 *
 * Private helpers
 *
 */

fn get_filtered_files_from_backend<'a>(module: &'a Module,
                                       env: &CommandEnv) -> IntoIter<File<'a>>
{
    let parser = Parser::new(JsonHeaderParser::new(None));
    let tags = get_tags(env.rt, env.matches);
    debug!("Tags: {:?}", tags);
    env.bk.iter_files(module, &parser).and_then(|files| {
        let f = files.filter(|file| {
            if tags.len() != 0 {
                debug!("Checking tags of: {:?}", file.id());
                get_tags_from_header(&file.header()).iter()
                    .any(|t| tags.contains(t))
            } else {
                true
            }
        }).filter(|file| {
            debug!("Checking matches of: {:?}", file.id());
            get_matcher(env.rt, env.matches)
                .and_then(|r| Some(file.matches_with(&r)))
                .unwrap_or(true)
        }).collect::<Vec<File>>();
        Some(f)
    }).unwrap_or(Vec::<File>::new()).into_iter()
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

/*
 * Checks whether the commandline call was set to run the removal "checked",
 * so if another entry from the store refers to this ID, do not remove the file.
 */
fn run_removal_checking(env: &CommandEnv) -> bool {
    env.matches.is_present("check")
}
