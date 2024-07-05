#[cfg_attr(mobile, tauri::mobile_entry_point)]
// use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;
use surrealdb::engine::remote::ws::Ws;
// use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
// use surrealdb::sql::Value;
use surrealdb::Surreal;
use std::borrow::Cow;
use tauri::command;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Serialize, Deserialize)]
struct Person {
    title: String,
    name: Name,
    marketing: bool,
}

#[derive(Serialize, Deserialize)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}


// static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);


#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[command]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[command]
async fn execute_query(query: String) -> Result<JsonValue, String> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.map_err(|e| e.to_string())?;

    // Sign in as a root user
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.map_err(|e| e.to_string())?;

    // Select a specific namespace and database
    db.use_ns("test").use_db("test").await.map_err(|e| e.to_string())?;

    // Execute the query
    let mut response = db.query("SELECT * FROM person").await.map_err(|e| e.to_string())?;

    // Debug print the response
    println!("Response: {:?}", response);

    // Process the response correctly
    let result: Vec<Person> = response.take(0).map_err(|e| e.to_string())?;

    // Debug print the result
    // dbg!(result);

    // Convert the result to JSON
    let json_result = serde_json::to_value(result).map_err(|e| e.to_string())?;
    println!("JsonResult: {:?}", json_result);

    Ok(json_result)
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, add, execute_query])
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
