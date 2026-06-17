use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub rom_path: String,
    pub box_art: String,
    pub developer: String,
    pub year: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchPlan {
    pub rom_path: String,
    pub core_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchResult {
    pub success: bool,
    pub message: String,
    pub plan: LaunchPlan,
    pub av_info: String,
}
