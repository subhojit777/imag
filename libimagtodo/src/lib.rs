extern crate semver;
extern crate uuid;

extern crate task_hookrs;
#[macro_use] extern crate libimagstore;

module_entry_path_mod!("todo", "0.1.0");

pub mod error;
pub mod task;
pub mod delete;
pub mod read;
pub mod set;
pub mod add;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
