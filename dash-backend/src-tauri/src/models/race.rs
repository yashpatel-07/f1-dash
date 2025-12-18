use serde::{Deserialize, Serialize};

// For Driver Standings API
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(rename = "MRData")]
    pub mr_data: MRData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MRData {
    pub race_table: RaceTable,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RaceTable {
    pub season: String,
    #[serde(rename = "Races")]
    pub races: Vec<Races>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Races {
    pub season: String,
    pub round: String,
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: String,
    pub time: String,
    #[serde(rename = "FirstPractice")]
    pub first_practice: Session,
    #[serde(rename = "SecondPractice")]
    pub second_practice: Option<Session>,
    #[serde(rename = "ThirdPractice")]
    pub third_practice: Option<Session>,
    #[serde(rename = "Qualifying")]
    pub qualifying: Session,
    #[serde(rename = "Sprint")]
    pub sprint: Option<Session>,
    #[serde(rename = "SprintQualifying")]
    pub sprint_qualifying: Option<Session>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]   
#[allow(dead_code)]
pub struct Circuit {
    pub circuit_id: String,
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Location {
    pub lat: String,
    pub long: String,
    pub locality: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub date: String,
    pub time: String,
}

// -- Simplified Struct for Frontend --
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RaceStatus {
    pub season: String,
    pub round: String,
    pub race_name: String,
    pub date: String,
    pub time: String,
    pub first_practice: Session,
    pub second_practice: Option<Session>,
    pub third_practice: Option<Session>,
    pub qualifying: Session,
    pub sprint: Option<Session>,
    pub sprint_qualifying: Option<Session>,
}