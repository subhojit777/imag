use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

pub mod spec;
pub mod data;

use self::data::*;
use self::spec::*;

pub struct MatchError<'a> {
    summary: String,
    expected: &'a FileHeaderSpec,
    found: &'a FileHeaderData
}

impl<'a> MatchError<'a> {

    pub fn new(s: String,
               ex: &'a FileHeaderSpec,
               found: &'a FileHeaderData) -> MatchError<'a> {
        MatchError {
            summary: s,
            expected: ex,
            found: found,
        }
    }

    pub fn format(&self) -> String {
        format!("MatchError: {:?}\nExpected: {:?}\nFound: {:?}\n",
               self.summary, self.expected, self.found)
    }
}

impl<'a> Error for MatchError<'a> {

    fn description(&self) -> &str {
        &self.summary[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl<'a> Debug for MatchError<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.format());
        Ok(())
    }

}

impl<'a> Display for MatchError<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.format());
        Ok(())
    }

}

pub fn match_header_spec<'a>(spec: &'a FileHeaderSpec, data: &'a FileHeaderData)
    -> Option<MatchError<'a>>
{
    debug!("Start matching:\n'{:?}'\non\n{:?}", spec, data);
    match (spec, data) {
        (&FileHeaderSpec::Null,     &FileHeaderData::Null)           => { }
        (&FileHeaderSpec::Bool,     &FileHeaderData::Bool(_))        => { }
        (&FileHeaderSpec::Integer,  &FileHeaderData::Integer(_))     => { }
        (&FileHeaderSpec::UInteger, &FileHeaderData::UInteger(_))    => { }
        (&FileHeaderSpec::Float,    &FileHeaderData::Float(_))       => { }
        (&FileHeaderSpec::Text,     &FileHeaderData::Text(_))        => { }

        (
            &FileHeaderSpec::Key{name: ref kname, value_type: ref vtype},
            &FileHeaderData::Key{name: ref n, value: ref val}
        ) => {
            debug!("Matching Key: '{:?}' == '{:?}', Value: '{:?}' == '{:?}'",
                    kname, n,
                    vtype, val);
            if kname != n {
                debug!("Keys not matching");
                unimplemented!();
            }
            return match_header_spec(&*vtype, &*val);
        }

        (
            &FileHeaderSpec::Map{keys: ref sks},
            &FileHeaderData::Map{keys: ref dks}
        ) => {
            debug!("Matching Map: '{:?}' == '{:?}'", sks, dks);

            for (s, d) in sks.iter().zip(dks.iter()) {
                let res = match_header_spec(s, d);
                if res.is_some() {
                    return res;
                }
            }
        }

        (
            &FileHeaderSpec::Array{allowed_types: ref vtypes},
            &FileHeaderData::Array{values: ref vs}
        ) => {
            debug!("Matching Array: '{:?}' == '{:?}'", vtypes, vs);
            for (t, v) in vtypes.iter().zip(vs.iter()) {
                let res = match_header_spec(t, v);
                if res.is_some() {
                    return res;
                }
            }
        }

        (k, v) => {
            return Some(MatchError::new(String::from("Expected type does not match found type"),
                                 k, v
                                 ))
        }
    }
    None
}

