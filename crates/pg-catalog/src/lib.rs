use std::collections::HashMap;

pub type TableId = u64;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub id: TableId,
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Default)]
pub struct Catalog {
    pub tables: HashMap<String, Table>,
}

impl Catalog {
    pub fn create_table(&mut self, name: String, columns: Vec<Column>) {
        let id = self.tables.len() as u64;
        self.tables.insert(name.clone(), Table { id, name, columns });
    }
}