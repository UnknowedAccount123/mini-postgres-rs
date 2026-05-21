use pg_planner::Plan;

pub fn execute(plan: Plan) -> String {
    match plan {
        Plan::CreateTable { name } => format!("table {} created", name),
        Plan::Insert { table } => format!("insert into {}", table),
        Plan::SeqScan { table } => format!("scan {}", table),
    }
}
