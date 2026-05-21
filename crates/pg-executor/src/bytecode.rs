#[derive(Debug, Clone)]
pub enum OpCode {
    Scan { table: String },
    Insert { table: String },
    CreateTable { name: String },
    FilterEq { column: String, value: String },
    Project { columns: Vec<String> },
    Halt,
}

pub struct BytecodeProgram {
    pub ops: Vec<OpCode>,
}

impl BytecodeProgram {
    pub fn new() -> Self {
        Self { ops: vec![] }
    }

    pub fn add(&mut self, op: OpCode) {
        self.ops.push(op);
    }
}
