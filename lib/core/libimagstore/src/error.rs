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

error_chain! {
    types {
        StoreError, StoreErrorKind, ResultExt, Result;
    }

    errors {

        ConfigurationError      {
            description("Store Configuration Error")
            display("Store Configuration Error")
        }

        ConfigTypeError         {
            description("Store configuration type error")
            display("Store configuration type error")
        }

        ConfigKeyMissingError   {
            description("Configuration Key missing")
            display("Configuration Key missing")
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

        IdNotFound              {
            description("ID not found")
            display("ID not found")
        }

        OutOfMemory             {
            description("Out of Memory")
            display("Out of Memory")
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

        StorePathExists         {
            description("Store path exists")
            display("Store path exists")
        }

        StorePathCreate         {
            description("Store path create")
            display("Store path create")
        }

        LockError               {
            description("Error locking datastructure")
            display("Error locking datastructure")
        }

        LockPoisoned            {
            description("The internal Store Lock has been poisoned")
            display("The internal Store Lock has been poisoned")
        }

        EntryAlreadyBorrowed    {
            description("Entry is already borrowed")
            display("Entry is already borrowed")
        }

        EntryAlreadyExists      {
            description("Entry already exists")
            display("Entry already exists")
        }

        MalformedEntry          {
            description("Entry has invalid formatting, missing header")
            display("Entry has invalid formatting, missing header")
        }

        HeaderPathSyntaxError   {
            description("Syntax error in accessor string")
            display("Syntax error in accessor string")
        }

        HeaderPathTypeFailure   {
            description("Header has wrong type for path")
            display("Header has wrong type for path")
        }

        HeaderKeyNotFound       {
            description("Header Key not found")
            display("Header Key not found")
        }

        HeaderTypeFailure       {
            description("Header type is wrong")
            display("Header type is wrong")
        }

        StorePathLacksVersion   {
            description("The supplied store path has no version part")
            display("The supplied store path has no version part")
        }

        GlobError               {
            description("glob() error")
            display("glob() error")
        }

        EncodingError           {
            description("Encoding error")
            display("Encoding error")
        }

        StorePathError          {
            description("Store Path error")
            display("Store Path error")
        }

        EntryRenameError        {
            description("Entry rename error")
            display("Entry rename error")
        }

        StoreIdHandlingError    {
            description("StoreId handling error")
            display("StoreId handling error")
        }

        StoreIdLocalPartAbsoluteError {
            description("StoreId 'id' part is absolute (starts with '/') which is not allowed")
            display("StoreId 'id' part is absolute (starts with '/') which is not allowed")
        }

        StoreIdBuildFromFullPathError {
            description("Building StoreId from full file path failed")
            display("Building StoreId from full file path failed")
        }

        StoreIdHasNoBaseError   {
            description("StoreId has no 'base' part")
            display("StoreId has no 'base' part")
        }

        CreateCallError            {
            description("Error when calling create()")
            display("Error when calling create()")
        }

        RetrieveCallError          {
            description("Error when calling retrieve()")
            display("Error when calling retrieve()")
        }

        GetCallError               {
            description("Error when calling get()")
            display("Error when calling get()")
        }

        GetAllVersionsCallError    {
            description("Error when calling get_all_versions()")
            display("Error when calling get_all_versions()")
        }

        RetrieveForModuleCallError {
            description("Error when calling retrieve_for_module()")
            display("Error when calling retrieve_for_module()")
        }

        UpdateCallError            {
            description("Error when calling update()")
            display("Error when calling update()")
        }

        RetrieveCopyCallError      {
            description("Error when calling retrieve_copy()")
            display("Error when calling retrieve_copy()")
        }

        DeleteCallError            {
            description("Error when calling delete()")
            display("Error when calling delete()")
        }

        MoveCallError              {
            description("Error when calling move()")
            display("Error when calling move()")
        }

        MoveByIdCallError          {
            description("Error when calling move_by_id()")
            display("Error when calling move_by_id()")
        }

        // Parser-related errors

        TOMLParserErrors    {
            description("Several TOML-Parser-Errors")
            display("Several TOML-Parser-Errors")
        }

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

impl IntoError for StoreErrorKind {
    type Target: StoreError;

    fn into_error(self) -> Self::Target {
        StoreError::from_kind(self)
    }

    fn into_error_with_cause(self, cause: Box<Error>) -> Self::Target {
        StoreError::from_kind(self)
    }
}
