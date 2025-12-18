mod models;

use crate::models::driver::{ApiResponse as DriverApiResponse, DriverStatus};
use crate::models::race::{ApiResponse as RaceApiResponse, RaceStatus};
use crate::models::constructor::{ApiResponse as ConstructorApiResponse, ConstructorStatus};

const DRIVERS_STANDINGS_API: &str = "http://api.jolpi.ca/ergast/f1/current/driverStandings.json";
const RACE_SCHEDULE_API: &str = "https://api.jolpi.ca/ergast/f1/2025/races.json";
const CONSTRUCTORS_STANDINGS_API: &str = "https://api.jolpi.ca/ergast/f1/2025/constructorstandings.json";

#[tauri::command]
async fn get_drivers_standings() -> Result<Vec<DriverStatus>, String> {
    // 1. Make the API call
    let client = reqwest::Client::new();
    let response = client
        .get(DRIVERS_STANDINGS_API)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?; // Handle reqwest error

    // 2. Check for successful status code
    let response = response
        .error_for_status()
        .map_err(|e| format!("API returned an error status: {}", e))?;
    

    // 3. Deserialize the full JSON into the top-level struct
    let api_response: DriverApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to deserialize API response: {}", e))?;

    let standings_table = api_response.mr_data.standings_table;

    // 4. Transform the nested data into the simplified DriverStatus struct
    let driver_standings_list = standings_table
        .standings_lists
        .into_iter() // Iterate over the StandingsLists array
        .next() // Get the first (and likely only) item
        .ok_or_else(|| "No standings list found in API response".to_string())? // Error if array is empty
        .driver_standings; // Get the inner DriverStandings array

    let leader_points = driver_standings_list
        .first()
        .map(|ds| ds.points.parse::<f32>().unwrap_or(0.0))
        .unwrap_or(0.0);

    let driver_status_list: Vec<DriverStatus> = driver_standings_list
        .into_iter()
        .map(|ds| {
            let constructor_name = ds.constructors.first().map(|c| c.name.clone()).unwrap_or_else(|| "Unknown Team".to_string());

            let points_str = ds.points.clone();
            let driver_points = points_str.parse::<f32>().unwrap_or(0.0);

            DriverStatus {
                position: ds.position,
                points: points_str,
                wins: ds.wins,
                driver_name: format!("{} {}", ds.driver.given_name, ds.driver.family_name),
                team_name: constructor_name,
                difference_to_leader: if leader_points > driver_points {
                    Some(format!("{:.1}", leader_points - driver_points))
                } else {
                    None
                },
            }
        })
        .collect();

    Ok(driver_status_list)
}

#[tauri::command]
async fn get_race_schedule() -> Result<Vec<RaceStatus>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(RACE_SCHEDULE_API)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let response = response
        .error_for_status()
        .map_err(|e| format!("API returned an error status: {}", e))?;

    let api_response: RaceApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to deserialize API response: {}", e))?;

    let race_table = api_response.mr_data.race_table;

    let race_status_list: Vec<RaceStatus> = race_table.races
        .into_iter()
        .map(|race| {
            RaceStatus { 
                season: race.season,
                round: race.round,
                race_name: race.race_name,
                date: race.date,
                time: race.time,
                qualifying: race.qualifying,
                first_practice: race.first_practice,
                second_practice: race.second_practice,
                third_practice: race.third_practice,
                sprint: race.sprint,
                sprint_qualifying: race.sprint_qualifying,
            }  
        })
        .collect();

    Ok(race_status_list)

}

#[tauri::command]
async fn get_constructors_standings() -> Result<Vec<ConstructorStatus>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(CONSTRUCTORS_STANDINGS_API)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?; // Handle reqwest error

    let response = response
        .error_for_status()
        .map_err(|e| format!("API returned an error status: {}", e))?;
    

    let api_response: ConstructorApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to deserialize API response: {}", e))?;

    let standings_table = api_response.mr_data.standings_table;

    let constructor_standings_list = standings_table
        .standings_lists
        .into_iter()
        .next()
        .ok_or_else(|| "No standings list found in API response".to_string())? // Error if array is empty
        .constructor_standings;

    let constructor_status_list: Vec<ConstructorStatus> = constructor_standings_list
        .into_iter()
        .map(|cs| {
            ConstructorStatus {
                position: cs.position,
                points: cs.points,
                wins: cs.wins,
                team_name: cs.constructor.name,
            }
        })
        .collect();

    Ok(constructor_status_list)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_drivers_standings, get_race_schedule, get_constructors_standings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
