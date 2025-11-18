use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(rename = "MRData")]
    pub mr_data: MRData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MRData {
    #[allow(non_snake_case)]
    #[serde(flatten)]
    pub kind: MRKind,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", untagged)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub enum MRKind {
    StandingsTable { StandingsTable: StandingsTable },
    RaceTable { RaceTable: RaceTable },
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

// For Race Schedule API
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

