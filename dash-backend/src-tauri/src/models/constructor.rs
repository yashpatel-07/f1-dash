use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ApiResponse {
    #[serde(rename = "MRData")]
    pub mr_data: MRData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct MRData {
    pub standings_table: StandingsTable,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct StandingsTable {
    pub season: String,
    pub round: String,
    #[serde(rename = "StandingsLists")]
    pub standings_lists: Vec<StandingsLists>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct StandingsLists {
    pub season: String,
    pub round: String,
    #[serde(rename = "ConstructorStandings")]
    pub constructor_standings: Vec<ConstructorStandings>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ConstructorStandings {
    pub position: String,
    #[serde(rename = "positionText")]
    pub position_text: String,
    pub points: String,
    pub wins: String,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Constructor {
    pub constructor_id: String,
    pub name: String,
    pub nationality: String,
    pub url: String,
}

// --- Simplified Struct for Frontend ---
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ConstructorStatus {
    pub position: String,
    pub points: String,
    pub wins: String,
    pub team_name: String,
}