// Needed for reading a Json File
// extern crate rustc_serialize;
// use rustc_serialize::json::Json;
// use std::fs::File;
// use std::io::Read;

use uuid::Uuid;

use libimagstore::store::Store;
use libimagstore::storeid::IntoStoreId;
use module_path::ModuleEntryPath;

use error::{TodoError, TodoErrorKind};

/// With the uuid we get the storeid and than we can delete the entry
pub fn deleteFunc(uuid: Uuid, store : &Store) {	
	// With this we can read from a .json File
	// let mut file = File::open("text.json").unwrap();
	// let mut data = String::new();
	// file.rad_to_string(&mut data).unwrap();
	//
	// let jeson = Json::from_str(&data).unwrap();
	// println!("{}", json.find_path(&["uuid"]).unwrap());		

	// With the uuid we get the storeid
	let store_id = ModuleEntryPath::new(format!("taskwarrior/{}", uuid)).into_storeid();
	// It deletes an entry	
	if let Err(e) = store.delete(store_id) {
		return Err(TodoError::new(TodoErrorKind::StoreError, Some(Box::new(e)))).unwrap();	
	}

}

