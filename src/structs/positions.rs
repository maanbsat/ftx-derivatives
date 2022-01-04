use serde::Deserialize;

use super::contract::Contract;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")] 
pub enum PositionType {
    Long,
    Short,
}

#[derive(Deserialize, Debug)]
pub struct Position {
    pub id: u64,
    pub size: i32,
    pub assigned_size: i32,
    #[serde(rename = "type")]
    pub position_type: PositionType,
    pub exercise_instruction: Option<String>,
    pub has_settled: bool,
    pub contract: Contract,
}
