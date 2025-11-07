#[derive(serde::Serialize)]
struct DriversStanding {
    driver: String,
    position: u8,
    points: String,
}

#[tauri::command]
fn get_drivers_standings() -> Vec<DriversStanding> {
    vec![
        DriversStanding {
            driver: "Lewis Hamilton".into(),
            position: 1,
            points: "256".into(),
        },
        DriversStanding {
            driver: "Max Verstappen".into(),
            position: 2,
            points: "249".into(),
        },
        DriversStanding {
            driver: "Valtteri Bottas".into(),
            position: 3,
            points: "186".into(),
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_drivers_standings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
