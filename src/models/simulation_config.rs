use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct SimulationConfig {
    process_id: Uuid,
    simulations: i32,
    ticks: i32,
    maximum_upgrades: i32,
    income_interval: i32,
    initial_balance: i32
}

