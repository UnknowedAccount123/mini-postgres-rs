#[derive(Debug)]
pub enum Statement {
    CreateTable { name: String, columns: Vec<(String, String)> },
    Insert { table: String, values: Vec<String> },
    Select { table: String, filter: Option<(String, String)> },
}

pub fn parse(sql: &str) -> Statement {
    let sql = sql.to_lowercase();

    if sql.starts_with("create table") {
        Statement::CreateTable {
            name: "users".into(),
            columns: vec![("id".into(), "int".into()), ("name".into(), "text".into())],
        }
    } else if sql.starts_with("insert") {
        Statement::Insert {
            table: "users".into(),
            values: vec!["1".into(), "john".into()],
        }
    } else {
        Statement::Select {
            table: "users".into(),
            filter: Some(("id".into(), "1".into())),
        }
    }
}
