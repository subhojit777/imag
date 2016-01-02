use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::process::exit;

use clap::ArgMatches;
use regex::Regex;

use storage::file::File;
use storage::file::hash::FileHash;
use storage::json::parser::JsonHeaderParser;
use storage::parser::FileHeaderParser;
use storage::parser::Parser;

pub trait CliFileFilter {

    fn filter_file(&self, &Rc<RefCell<File>>) -> bool;

    fn not(self) -> CliFileFilterNot
        where Self: Sized + 'static
    {
        CliFileFilterNot {
            a: Box::new(self),
        }
    }

    fn or(self, other: Box<CliFileFilter>) -> CliFileFilterOr
        where Self: Sized + 'static
    {
        CliFileFilterOr {
            a: Box::new(self),
            b: other
        }
    }

    fn and(self, other: Box<CliFileFilter>) -> CliFileFilterAnd
        where Self: Sized + 'static
    {
        CliFileFilterAnd {
            a: Box::new(self),
            b: other
        }
    }

}

pub struct CliFileFilterNot {
    a: Box<CliFileFilter>,
}

impl CliFileFilter for CliFileFilterNot {

    fn filter_file(&self, f: &Rc<RefCell<File>>) -> bool {
        !self.a.filter_file(f)
    }

}

pub struct CliFileFilterOr {
    a: Box<CliFileFilter>,
    b: Box<CliFileFilter>
}

impl CliFileFilter for CliFileFilterOr {

    fn filter_file(&self, f: &Rc<RefCell<File>>) -> bool {
        self.a.filter_file(f) || self.b.filter_file(f)
    }

}

pub struct CliFileFilterAnd {
    a: Box<CliFileFilter>,
    b: Box<CliFileFilter>
}

impl CliFileFilter for CliFileFilterAnd {

    fn filter_file(&self, f: &Rc<RefCell<File>>) -> bool {
        self.a.filter_file(f) && self.b.filter_file(f)
    }

}

pub struct CliFileFilterByHash {
    default: bool,
    hash: Option<FileHash>,
}

impl CliFileFilter for CliFileFilterByHash {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        self.hash.clone().map(|h| {
            debug!("Filtering file with hash = {}", h);
            let f = file.deref().borrow();
            f.id().get_id() == h
        })
        .unwrap_or(self.default)
    }

}

pub struct CliFileFilterByDataRegex {
    default: bool,
    regex: Option<Regex>,
}

impl CliFileFilter for CliFileFilterByDataRegex {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        self.regex.clone().map(|r| {
            debug!("Filtering file with regex = {:?}", r);
            let f = file.deref().borrow();
            r.is_match(&f.data()[..])
        })
        .unwrap_or(self.default)
    }

}

pub struct CliFileFilterByHeaderRegex {
    default: bool,
    header_field_name: &'static str,
    regex: Option<Regex>,
}

impl CliFileFilter for CliFileFilterByHeaderRegex {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        use module::helpers::header::data::get_named_text_from_header;

        self.regex.clone().map(|r| {
            debug!("Filtering file (header field = {}) with regex = {:?}", self.header_field_name, r);

            let f = file.deref().borrow();
            get_named_text_from_header(self.header_field_name, f.header())
                .map(|headerfield| r.is_match(&headerfield[..]))
                .unwrap_or(self.default)
        })
        .unwrap_or(self.default)
    }

}

pub struct CliFileFilterByTags {
    default: bool,
    tags: Option<Vec<String>>,
}

impl CliFileFilter for CliFileFilterByTags {

    fn filter_file(&self, file: &Rc<RefCell<File>>) -> bool {
        use module::helpers::header::tags::data::get_tags_from_header;

        self.tags.clone().map(|ts| {
            debug!("Filtering file with tags = {:?}", ts);

            let f = file.deref().borrow();
            get_tags_from_header(f.header())
                .iter()
                .any(|tag| ts.iter().any(|remtag| remtag == tag))
        })
        .unwrap_or(self.default)
    }

}

/*
 *
 *
 * Functions to generate filters
 *
 *
 */

pub fn create_hash_filter(matches: &ArgMatches, id_key: &'static str, default: bool) -> CliFileFilterByHash {
    CliFileFilterByHash {
        hash: matches.value_of(id_key).map(FileHash::from),
        default: default
    }
}

pub fn create_content_grep_filter(matches: &ArgMatches, match_key: &'static str, default: bool) -> CliFileFilterByDataRegex {
    use std::process::exit;

     CliFileFilterByDataRegex {
        regex: matches.value_of(match_key).map(|m| {
            Regex::new(&m[..]).unwrap_or_else(|e| {
                error!("Regex compiler error: {}", e);
                exit(1);
            })
        }),
        default: default,
     }
}

pub fn create_text_header_field_grep_filter(matches: &ArgMatches,
                                            match_key: &'static str,
                                            header_field_name: &'static str,
                                            default: bool)
    -> CliFileFilterByHeaderRegex
{
    CliFileFilterByHeaderRegex {
        default: default,
        header_field_name: header_field_name,
        regex: matches.value_of(match_key)
                      .map(|m| {
                        Regex::new(&m[..]).unwrap_or_else(|e| {
                            error!("Regex compiler error: {}", e);
                            exit(1);
                        })
                      }),
    }
}

pub fn create_tag_filter(matches: &ArgMatches, tag_key: &'static str, default: bool) -> CliFileFilterByTags {

    CliFileFilterByTags {
        default: default,
        tags: matches.value_of(tag_key)
                     .map(|m| m.split(",")
                               .map(String::from)
                               .collect::<Vec<String>>()
                     ),
    }
}

