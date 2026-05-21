use pg_parser::Statement;

pub enum Plan {
    SeqScan { table: String },
    Insert { table: String },
    CreateTable { name: String },
}

pub fn plan(stmt: Statement) -> Plan {
    match stmt {
        Statement::CreateTable { name, .. } => Plan::CreateTable { name },
        Statement::Insert { table, .. } => Plan::Insert { table },
        Statement::Select { table, .. } => Plan::SeqScan { table },
    }
}
