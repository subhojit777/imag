use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::process::exit;

use clap::ArgMatches;
use regex::Regex;

use storage::file::File;
use storage::file::hash::FileHash;
use storage::file::header::data::FileHeaderData;
use storage::file::id::FileID;
use storage::json::parser::JsonHeaderParser;
use storage::parser::FileHeaderParser;
use storage::parser::Parser;

pub trait CliFileFilter {

    fn filter_file(&self, &Rc<RefCell<File>>) -> bool;

}

struct CliFileFilterDefault {
    default: bool,
}

impl CliFileFilter for CliFileFilterDefault {

    fn filter_file(&self, _: &Rc<RefCell<File>>) -> bool {
        debug!("Filtering file with default value = {}", self.default);
        return self.default
    }

}

struct CliFileFilterByHash {
    hash: FileHash,
}

impl CliFileFilter for CliFileFilterByHash {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        debug!("Filtering file with hash = {}", self.hash);
        let f = file.deref().borrow();
        f.id().get_id() == self.hash
    }

}

struct CliFileFilterByDataRegex {
    regex: Regex,
}

impl CliFileFilter for CliFileFilterByDataRegex {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        debug!("Filtering file with regex = {:?}", self.regex);
        let f = file.deref().borrow();
        self.regex.is_match(&f.data()[..])
    }

}

struct CliFileFilterByHeaderRegex {
    header_field_name: &'static str,
    regex: Regex,
}

impl CliFileFilter for CliFileFilterByHeaderRegex {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        use module::helpers::header::data::get_named_text_from_header;

        debug!("Filtering file (header field = {}) with regex = {:?}",
               self.header_field_name,
               self.regex);

        let f = file.deref().borrow();
        get_named_text_from_header(self.header_field_name, f.header())
            .map(|headerfield| self.regex.is_match(&headerfield[..]))
            .unwrap_or(false)
    }

}

struct CliFileFilterByTags {
    tags: Vec<String>,
}

impl CliFileFilter for CliFileFilterByTags {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        use module::helpers::header::tags::data::get_tags_from_header;

        debug!("Filtering file with tags = {:?}", self.tags);

        let f = file.deref().borrow();
        get_tags_from_header(f.header())
            .iter()
            .any(|tag| self.tags.iter().any(|remtag| remtag == tag))
    }

}

/**
 * Helper function to get files from the store filtered by the constraints passed via the
 * CLI
 */
pub fn get_file_filter_by_cli<HP>(parser: &Parser<HP>,
                                  matches: &ArgMatches,
                                  id_key:    &'static str,
                                  match_key: &'static str,
                                  tag_key:   &'static str,
                                  header_field_name: Option<&'static str>)
    -> Box<CliFileFilter>
    where HP: FileHeaderParser,
{
    if matches.is_present(id_key) {
        Box::new(CliFileFilterByHash { hash: FileHash::from(matches.value_of(id_key).unwrap()) })
    } else if matches.is_present(match_key) {
        let matcher = String::from(matches.value_of(match_key).unwrap());
        header_field_name
            .and_then(|header_field_name| {
                Some(get_files_by_header_field_match_filter(parser,
                                                     &matcher,
                                                     header_field_name))
            })
            .unwrap_or(get_file_by_match_filter(parser, &matcher))
    } else if matches.is_present(tag_key) {
        let tags = matches.value_of(tag_key)
                          .unwrap()
                          .split(",")
                          .map(String::from)
                          .collect::<Vec<String>>();
        get_file_by_tags_filter(tags)
    } else {
        Box::new(CliFileFilterDefault { default: true })
    }
}

/**
 * Get files from the store, filtere by Regex
 */
fn get_file_by_match_filter<HP>(parser: &Parser<HP>, matcher: &String)
    -> Box<CliFileFilter>
    where HP: FileHeaderParser
{
    let parser = Parser::new(JsonHeaderParser::new(None));
    let re = Regex::new(&matcher[..]).unwrap_or_else(|e| {
        error!("Cannot build regex out of '{}'", matcher);
        error!("{}", e);
        exit(1);
    });

    debug!("Compiled '{}' to regex: '{:?}'", matcher, re);

    Box::new(CliFileFilterByDataRegex { regex: re })
}

fn get_files_by_header_field_match_filter<HP>(parser: &Parser<HP>,
                                       matcher: &String,
                                       header_field_name: &'static str)
    -> Box<CliFileFilter>
    where HP: FileHeaderParser,
{
    let parser = Parser::new(JsonHeaderParser::new(None));
    let re = Regex::new(&matcher[..]).unwrap_or_else(|e| {
        error!("Cannot build regex out of '{}'", matcher);
        error!("{}", e);
        exit(1);
    });

    debug!("Compiled '{}' to regex: '{:?}'", matcher, re);

    Box::new(CliFileFilterByHeaderRegex {
        header_field_name: header_field_name,
        regex: re
    })
}

/**
 * Get files from the store, filtere by tags
 */
fn get_file_by_tags_filter(tags: Vec<String>)
    -> Box<CliFileFilter>
{
    Box::new(CliFileFilterByTags { tags: tags })
}

