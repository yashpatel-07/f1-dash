use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(rename = "MRData")]
    pub mr_data: MRData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
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
    #[serde(rename = "DriverStandings")] 
    pub driver_standings: Vec<DriverStandings>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct DriverStandings {
    pub position: String,
    pub points: String,
    pub wins: String,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructors>, 
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Driver {
    pub driver_id: String,
    pub given_name: String,
    pub family_name: String,
    #[serde(default)] 
    pub permanent_number: Option<String>,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Constructors {
    pub constructor_id: String,
    pub name: String,
    pub nationality: String,
    pub url: String,
}

// --- Simplified Struct for Frontend ---
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DriverStatus {
    pub position: String,
    pub points: String,
    pub wins: String,
    pub driver_name: String,
    pub team_name: String,
    pub difference_to_leader: Option<String>,
}