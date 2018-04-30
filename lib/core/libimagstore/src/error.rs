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

use std::path::PathBuf;

use storeid::StoreId;

error_chain! {
    types {
        StoreError, StoreErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        Fmt(::std::fmt::Error);
        TomlDeserError(::toml::de::Error);
        TomlSerError(::toml::ser::Error);
        GlobPatternError(::glob::PatternError);
        TomlQueryError(::toml_query::error::Error);
    }

    errors {

        ConfigurationError      {
            description("Store Configuration Error")
            display("Store Configuration Error")
        }

        ConfigTypeError(key: &'static str, expected: &'static str) {
            description("Store configuration type error")
            display("Store configuration type error at '{}', expected {}", key, expected)
        }

        ConfigKeyMissingError(key: &'static str) {
            description("Configuration Key missing")
            display("Configuration Key missing: '{}'", key)
        }

        VersionError            {
            description("Incompatible store versions detected")
            display("Incompatible store versions detected")
        }

        CreateStoreDirDenied    {
            description("Creating store directory implicitely denied")
            display("Creating store directory implicitely denied")
        }

        FileError               {
            description("File Error")
            display("File Error")
        }

        IoError                 {
            description("IO Error")
            display("IO Error")
        }

        IdLocked                {
            description("ID locked")
            display("ID locked")
        }

        IdNotFound(sid: StoreId) {
            description("ID not found")
            display("ID not found: {}", sid)
        }

        FileNotFound            {
            description("File corresponding to ID not found")
            display("File corresponding to ID not found")
        }

        FileNotCreated          {
            description("File corresponding to ID could not be created")
            display("File corresponding to ID could not be created")
        }

        FileNotWritten          {
            description("File corresponding to ID could not be written to")
            display("File corresponding to ID could not be written to")
        }

        FileNotSeeked           {
            description("File corresponding to ID could not be seeked")
            display("File corresponding to ID could not be seeked")
        }

        FileNotRemoved          {
            description("File corresponding to ID could not be removed")
            display("File corresponding to ID could not be removed")
        }

        FileNotRenamed          {
            description("File corresponding to ID could not be renamed")
            display("File corresponding to ID could not be renamed")
        }

        FileNotCopied           {
            description("File could not be copied")
            display("File could not be copied")
        }

        DirNotCreated           {
            description("Directory/Directories could not be created")
            display("Directory/Directories could not be created")
        }

        StorePathExists(pb: PathBuf) {
            description("Store path exists")
            display("Store path exists: {:?}", pb)
        }

        StorePathCreate(pb: PathBuf) {
            description("Store path create")
            display("Store path create: {:?}", pb)
        }

        LockError               {
            description("Error locking datastructure")
            display("Error locking datastructure")
        }

        LockPoisoned            {
            description("The internal Store Lock has been poisoned")
            display("The internal Store Lock has been poisoned")
        }

        EntryAlreadyBorrowed(id: StoreId) {
            description("Entry is already borrowed")
            display("Entry is already borrowed: {:?}", id)
        }

        EntryAlreadyExists(id: StoreId) {
            description("Entry already exists")
            display("Entry already exists: {:?}", id)
        }

        MalformedEntry          {
            description("Entry has invalid formatting, missing header")
            display("Entry has invalid formatting, missing header")
        }

        HeaderTypeFailure       {
            description("Header type is wrong")
            display("Header type is wrong")
        }

        EncodingError           {
            description("Encoding error")
            display("Encoding error")
        }

        EntryRenameError(old: PathBuf, new: PathBuf) {
            description("Entry rename error")
            display("Entry rename error: {:?} -> {:?}", old, new)
        }

        StoreIdHandlingError    {
            description("StoreId handling error")
            display("StoreId handling error")
        }

        StoreIdLocalPartAbsoluteError(pb: PathBuf) {
            description("StoreId 'id' part is absolute (starts with '/') which is not allowed")
            display("StoreId 'id' part is absolute (starts with '/') which is not allowed: {:?}", pb)
        }

        StoreIdBuildFromFullPathError {
            description("Building StoreId from full file path failed")
            display("Building StoreId from full file path failed")
        }

        StoreIdHasNoBaseError(pb: PathBuf) {
            description("StoreId has no 'base' part")
            display("StoreId has no 'base' part: {:?}", pb)
        }

        CreateCallError(sid: StoreId) {
            description("Error when calling create()")
            display("Error when calling create({:?})", sid)
        }

        RetrieveCallError(sid: StoreId) {
            description("Error when calling retrieve()")
            display("Error when calling retrieve({:?})", sid)
        }

        GetCallError(sid: StoreId) {
            description("Error when calling get()")
            display("Error when calling get({:?})", sid)
        }

        UpdateCallError(sid: StoreId) {
            description("Error when calling update()")
            display("Error when calling update({:?})", sid)
        }

        RetrieveCopyCallError(sid: StoreId) {
            description("Error when calling retrieve_copy()")
            display("Error when calling retrieve_copy({:?})", sid)
        }

        DeleteCallError(sid: StoreId) {
            description("Error when calling delete()")
            display("Error when calling delete({:?})", sid)
        }

        MoveCallError(old: StoreId, new: StoreId) {
            description("Error when calling move()")
            display("Error when calling move({:?} -> {:?})", old, new)
        }

        // Parser-related errors

        MissingMainSection  {
            description("Missing main section")
            display("Missing main section")
        }

        MissingVersionInfo  {
            description("Missing version information in main section")
            display("Missing version information in main section")
        }

        NonTableInBaseTable {
            description("A non-table was found in the base table")
            display("A non-table was found in the base table")
        }

        HeaderInconsistency {
            description("The header is inconsistent")
            display("The header is inconsistent")
        }
    }
}

