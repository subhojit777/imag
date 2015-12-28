use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File as FSFile;
use std::ops::Deref;
use std::io::Write;
use std::io::Read;

pub mod path;
pub mod file;
pub mod parser;
pub mod json;

use module::Module;
use runtime::Runtime;
use storage::file::File;
use storage::file::id::FileID;
use storage::file::id_type::FileIDType;
use storage::file::hash::FileHash;
use storage::parser::{FileHeaderParser, Parser, ParserError};
use storage::file::header::data::FileHeaderData;

type Cache = HashMap<FileID, Rc<RefCell<File>>>;

pub struct Store {
    storepath: String,
    cache : RefCell<Cache>,
}

impl Store {

    pub fn new(storepath: String) -> Store {
        Store {
            storepath: storepath,
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn put_in_cache(&self, f: File) -> FileID {
        let res = f.id().clone();
        self.cache.borrow_mut().insert(f.id().clone(), Rc::new(RefCell::new(f)));
        res
    }

    pub fn load_in_cache<HP>(&self, m: &Module, parser: &Parser<HP>, id: FileID)
        -> Option<Rc<RefCell<File>>>
        where HP: FileHeaderParser
    {
        let idstr : String = id.clone().into();
        let path = format!("{}/{}-{}.imag", self.storepath, m.name(), idstr);
        let mut string = String::new();

        FSFile::open(&path).map(|mut file| {
            file.read_to_string(&mut string)
                .map_err(|e| error!("Failed reading file: '{}'", path));
        });

        parser.read(string).map(|(header, data)| {
            self.new_file_from_parser_result(m, id.clone(), header, data);
        });

        self.load(&id)
    }

    pub fn new_file(&self, module: &Module)
        -> FileID
    {
        let f = File {
            owning_module_name: module.name(),
            header: FileHeaderData::Null,
            data: String::from(""),
            id: self.get_new_file_id(),
        };

        debug!("Create new File object: {:?}", &f);
        self.put_in_cache(f)
    }

    pub fn new_file_from_parser_result(&self,
                                       module: &Module,
                                       id: FileID,
                                       header: FileHeaderData,
                                       data: String)
        -> FileID
    {
        let f = File {
            owning_module_name: module.name(),
            header: header,
            data: data,
            id: id,
        };
        debug!("Create new File object from parser result: {:?}", f);
        self.put_in_cache(f)
    }

    pub fn new_file_with_header(&self,
                                module: &Module,
                                h: FileHeaderData)
        -> FileID
    {
        let f = File {
            owning_module_name: module.name(),
            header: h,
            data: String::from(""),
            id: self.get_new_file_id(),
        };
        debug!("Create new File object with header: {:?}", f);
        self.put_in_cache(f)
    }

    pub fn new_file_with_data(&self, module: &Module, d: String)
        -> FileID
    {
        let f = File {
            owning_module_name: module.name(),
            header: FileHeaderData::Null,
            data: d,
            id: self.get_new_file_id(),
        };
        debug!("Create new File object with data: {:?}", f);
        self.put_in_cache(f)
    }

    pub fn new_file_with_content(&self,
                                 module: &Module,
                                 h: FileHeaderData,
                                 d: String)
        -> FileID
    {
        let f = File {
            owning_module_name: module.name(),
            header: h,
            data: d,
            id: self.get_new_file_id(),
        };
        debug!("Create new File object with content: {:?}", f);
        self.put_in_cache(f)
    }

    pub fn persist<HP>(&self,
                       p: &Parser<HP>,
                       f: Rc<RefCell<File>>) -> bool
        where HP: FileHeaderParser
    {
        let file = f.deref().borrow();
        let text = p.write(file.contents());
        if text.is_err() {
            error!("Error: {}", text.err().unwrap());
            return false;
        }

        let path = {
            let ids : String = file.id().clone().into();
            format!("{}/{}-{}.imag", self.storepath, file.owning_module_name, ids)
        };

        self.ensure_store_path_exists();

        FSFile::create(&path).map(|mut fsfile| {
            fsfile.write_all(&text.unwrap().clone().into_bytes()[..])
        }).map_err(|writeerr|  {
            debug!("Could not create file at '{}'", path);
        }).and(Ok(true)).unwrap()
    }

    fn ensure_store_path_exists(&self) {
        use std::fs::create_dir_all;
        use std::process::exit;

        create_dir_all(&self.storepath).unwrap_or_else(|e| {
            error!("Could not create store: '{}'", self.storepath);
            error!("Error                 : '{}'", e);
            error!("Killing myself now");
            exit(1);
        })
    }

    pub fn load(&self, id: &FileID) -> Option<Rc<RefCell<File>>> {
        debug!("Loading '{:?}'", id);
        self.cache.borrow().get(id).cloned()
    }

    pub fn remove(&self, id: FileID) -> bool {
        use std::fs::remove_file;

        self.cache
            .borrow_mut()
            .remove(&id)
            .map(|file| {
                let idstr : String = id.into();
                let path = format!("{}/{}-{}.imag",
                                   self.storepath,
                                   file.deref().borrow().owner_name(),
                                   idstr);
                remove_file(path).is_ok()
            })
            .unwrap_or(false)
    }

    pub fn load_for_module<HP>(&self, m: &Module, parser: &Parser<HP>)
        -> Vec<Rc<RefCell<File>>>
        where HP: FileHeaderParser
    {
        use glob::{glob, Paths, PatternError};

        let globstr = format!("{}/{}-*.imag", self.storepath, m.name());
        let mut res = vec![];

        glob(&globstr[..]).map(|paths| {
            for path in paths {
                if let Ok(pathbuf) = path {
                    let fname = pathbuf.file_name().and_then(|s| s.to_str());
                    fname.map(|s| {
                        FileID::parse(&String::from(s)).map(|id| {
                            self.load_in_cache(m, parser, id).map(|file| {
                                res.push(file);
                            })
                        });
                    });
                }
            }
        });
        res
    }

    fn get_new_file_id(&self) -> FileID {
        use uuid::Uuid;
        let hash = FileHash::from(Uuid::new_v4().to_hyphenated_string());
        FileID::new(FileIDType::UUID, hash)
    }

}
