use crate::bytecode::{BytecodeProgram, OpCode};

pub struct VDBE {
    pub ip: usize,
}

impl VDBE {
    pub fn new() -> Self {
        Self { ip: 0 }
    }

    pub fn run(&mut self, program: BytecodeProgram) -> String {
        let mut output = String::new();

        for op in program.ops {
            match op {
                OpCode::Scan { table } => output.push_str(&format!("SCAN {}\n", table)),
                OpCode::Insert { table } => output.push_str(&format!("INSERT {}\n", table)),
                OpCode::CreateTable { name } => output.push_str(&format!("CREATE {}\n", name)),
                OpCode::FilterEq { column, value } => output.push_str(&format!("FILTER {}={}\n", column, value)),
                OpCode::Project { columns } => output.push_str(&format!("PROJECT {:?}\n", columns)),
                OpCode::Halt => output.push_str("HALT\n"),
            }
        }

        output
    }
}
