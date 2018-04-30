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

use std::collections::BTreeMap;
use std::io::BufRead;
use std::result::Result as RResult;

use toml::Value;
use uuid::Uuid;

use task_hookrs::task::Task as TTask;
use task_hookrs::import::{import_task, import_tasks};

use libimagstore::store::{FileLockEntry, Store};
use libimagstore::storeid::IntoStoreId;
use module_path::ModuleEntryPath;

use error::TodoErrorKind as TEK;
use error::TodoError as TE;
use error::Result;
use error::ResultExt;
use iter::TaskIdIterator;

/// Task struct containing a `FileLockEntry`
pub trait TaskStore<'a> {
    fn import_task_from_reader<R: BufRead>(&'a self, r: R) -> Result<(FileLockEntry<'a>, String, Uuid)>;
    fn get_task_from_import<R: BufRead>(&'a self, r: R) -> Result<RResult<FileLockEntry<'a>, String>>;
    fn get_task_from_string(&'a self, s: String) -> Result<RResult<FileLockEntry<'a>, String>>;
    fn get_task_from_uuid(&'a self, uuid: Uuid) -> Result<Option<FileLockEntry<'a>>>;
    fn retrieve_task_from_import<R: BufRead>(&'a self, r: R) -> Result<FileLockEntry<'a>>;
    fn retrieve_task_from_string(&'a self, s: String) -> Result<FileLockEntry<'a>>;
    fn delete_tasks_by_imports<R: BufRead>(&self, r: R) -> Result<()>;
    fn delete_task_by_uuid(&self, uuid: Uuid) -> Result<()>;
    fn all_tasks(&self) -> Result<TaskIdIterator>;
    fn new_from_twtask(&'a self, task: TTask) -> Result<FileLockEntry<'a>>;
}

impl<'a> TaskStore<'a> for Store {

    fn import_task_from_reader<R: BufRead>(&'a self, mut r: R) -> Result<(FileLockEntry<'a>, String, Uuid)> {
        let mut line = String::new();
        r.read_line(&mut line).map_err(|_| TE::from_kind(TEK::UTF8Error))?;
        import_task(&line.as_str())
            .map_err(|_| TE::from_kind(TEK::ImportError))
            .and_then(|t| {
                let uuid = t.uuid().clone();
                self.new_from_twtask(t).map(|t| (t, line, uuid))
            })
    }

    /// Get a task from an import string. That is: read the imported string, get the UUID from it
    /// and try to load this UUID from store.
    ///
    /// Possible return values are:
    ///
    /// * Ok(Ok(Task))
    /// * Ok(Err(String)) - where the String is the String read from the `r` parameter
    /// * Err(_)          - where the error is an error that happened during evaluation
    ///
    fn get_task_from_import<R: BufRead>(&'a self, mut r: R) -> Result<RResult<FileLockEntry<'a>, String>> {
        let mut line = String::new();
        r.read_line(&mut line).chain_err(|| TEK::UTF8Error)?;
        self.get_task_from_string(line)
    }

    /// Get a task from a String. The String is expected to contain the JSON-representation of the
    /// Task to get from the store (only the UUID really matters in this case)
    ///
    /// For an explanation on the return values see `Task::get_from_import()`.
    fn get_task_from_string(&'a self, s: String) -> Result<RResult<FileLockEntry<'a>, String>> {
        import_task(s.as_str())
            .map_err(|_| TE::from_kind(TEK::ImportError))
            .map(|t| t.uuid().clone())
            .and_then(|uuid| self.get_task_from_uuid(uuid))
            .and_then(|o| match o {
                None    => Ok(Err(s)),
                Some(t) => Ok(Ok(t)),
            })
    }

    /// Get a task from an UUID.
    ///
    /// If there is no task with this UUID, this returns `Ok(None)`.
    fn get_task_from_uuid(&'a self, uuid: Uuid) -> Result<Option<FileLockEntry<'a>>> {
        ModuleEntryPath::new(format!("taskwarrior/{}", uuid))
            .into_storeid()
            .and_then(|store_id| self.get(store_id))
            .map_err(TE::from)
    }

    /// Same as Task::get_from_import() but uses Store::retrieve() rather than Store::get(), to
    /// implicitely create the task if it does not exist.
    fn retrieve_task_from_import<R: BufRead>(&'a self, mut r: R) -> Result<FileLockEntry<'a>> {
        let mut line = String::new();
        r.read_line(&mut line).chain_err(|| TEK::UTF8Error)?;
        self.retrieve_task_from_string(line)
    }

    /// Retrieve a task from a String. The String is expected to contain the JSON-representation of
    /// the Task to retrieve from the store (only the UUID really matters in this case)
    fn retrieve_task_from_string(&'a self, s: String) -> Result<FileLockEntry<'a>> {
        self.get_task_from_string(s)
            .and_then(|opt| match opt {
                Ok(task)    => Ok(task),
                Err(string) => import_task(string.as_str())
                    .map_err(|_| TE::from_kind(TEK::ImportError))
                    .and_then(|t| self.new_from_twtask(t)),
            })
    }

    fn delete_tasks_by_imports<R: BufRead>(&self, r: R) -> Result<()> {
        use serde_json::ser::to_string as serde_to_string;
        use task_hookrs::status::TaskStatus;

        for (counter, res_ttask) in import_tasks(r).into_iter().enumerate() {
            match res_ttask {
                Ok(ttask) => {
                    if counter % 2 == 1 {
                        // Only every second task is needed, the first one is the
                        // task before the change, and the second one after
                        // the change. The (maybe modified) second one is
                        // expected by taskwarrior.
                        match serde_to_string(&ttask).chain_err(|| TEK::ImportError) {
                            // use println!() here, as we talk with TW
                            Ok(val) => println!("{}", val),
                            Err(e)  => return Err(e),
                        }

                        // Taskwarrior does not have the concept of deleted tasks, but only modified
                        // ones.
                        //
                        // Here we check if the status of a task is deleted and if yes, we delete it
                        // from the store.
                        if *ttask.status() == TaskStatus::Deleted {
                            match self.delete_task_by_uuid(*ttask.uuid()) {
                                Ok(_)  => info!("Deleted task {}", *ttask.uuid()),
                                Err(e) => return Err(e),
                            }
                        }
                    } // end if c % 2
                },
                Err(_) => return Err(TE::from_kind(TEK::ImportError)),
            }
        }
        Ok(())
    }

    fn delete_task_by_uuid(&self, uuid: Uuid) -> Result<()> {
        ModuleEntryPath::new(format!("taskwarrior/{}", uuid))
            .into_storeid()
            .and_then(|id| self.delete(id))
            .map_err(TE::from)
    }

    fn all_tasks(&self) -> Result<TaskIdIterator> {
        self.entries()
            .map(|i| TaskIdIterator::new(i.without_store()))
            .map_err(TE::from)
    }

    fn new_from_twtask(&'a self, task: TTask) -> Result<FileLockEntry<'a>> {
        use toml_query::read::TomlValueReadExt;
        use toml_query::set::TomlValueSetExt;

        let uuid     = task.uuid();
        ModuleEntryPath::new(format!("taskwarrior/{}", uuid))
            .into_storeid()
            .chain_err(|| TEK::StoreIdError)
            .and_then(|id| {
                self.retrieve(id)
                    .map_err(TE::from)
                    .and_then(|mut fle| {
                        {
                            let hdr = fle.get_header_mut();
                            if hdr.read("todo")?.is_none() {
                                hdr.set("todo", Value::Table(BTreeMap::new()))?;
                            }

                            hdr.set("todo.uuid", Value::String(format!("{}",uuid)))?;
                        }

                        // If none of the errors above have returned the function, everything is fine
                        Ok(fle)
                    })
            })

    }

}

