use pg_planner::Plan;

pub struct VDBE;

impl VDBE {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, plan: Plan) -> String {
        match plan {
            Plan::CreateTable { name } => format!("[VDBE] create table {}", name),
            Plan::Insert { table } => format!("[VDBE] insert into {}", table),
            Plan::SeqScan { table } => format!("[VDBE] scan {}", table),
        }
    }
}
