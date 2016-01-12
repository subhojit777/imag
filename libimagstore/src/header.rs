use toml::Table;

pub struct EntryHeader {
    toml: Table,
}

impl EntryHeader {

    pub fn new(toml: Table) -> EntryHeader {
        EntryHeader {
            toml: toml,
        }
    }

    pub fn toml(&self) -> &Table {
        &self.toml
    }

}
