use serde::{Deserialize,Serialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct SimulationFrame {
    pub simulation_id: Uuid,
    pub frame_id: i32,
    pub income: i32,
    pub balance: i32,
    pub income_upgrade_cost: i32,
    pub income_upgrade_counts: i32
}

