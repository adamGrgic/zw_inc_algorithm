use serde::{Deserialize,Serialize};
use crate::models::SimulationFrame; // Bring models into scope
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SimulationResult {
    pub simulation_id: Uuid,
    pub values: Vec<SimulationFrame>,
}

