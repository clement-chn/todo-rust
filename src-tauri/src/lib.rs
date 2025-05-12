use serde::Serialize;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize)]
struct ToDo {
    todo_name: String,
    priority: String,
}

#[tauri::command]
fn save_name_to_json(todo_name: String, priority: String) {
    println!("todo_name: {}", todo_name);
    println!("priority: {}", priority);

    let todo = ToDo {
        todo_name,
        priority,
    };

    let new_todo_json = match serde_json::to_string(&todo) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing to JSON: {}", e);
            return;
        }
    };

    let mut file = match OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("todo.json")
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        eprintln!("Error reading file: {}", e);
        return;
    }

    let updated_content = if content.trim().is_empty() {
        format!("[{}]", new_todo_json)
    } else {
        let trimmed_content = content.trim_end_matches(']');
        format!("{},{}]", trimmed_content, new_todo_json)
    };

    if let Err(e) = std::fs::write("todo.json", updated_content) {
        eprintln!("Erreur lors de l'écriture dans le fichier : {}", e);
    } else {
        println!("Enregistrement effectué");
    }
}

#[tauri::command]
fn get_todos() -> Result<String, String> {
    match std::fs::read_to_string("todo.json") {
        Ok(content) => Ok(content),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err("Erreur lors de la lecture du fichier".to_string())
        }
    }
}

#[tauri::command]
fn delete_todo(index: usize) -> Result<(), String> {
    let content = match std::fs::read_to_string("todo.json") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return Err("Erreur lors de la lecture du fichier".to_string());
        }
    };

    let mut todos: Vec<ToDo> = match serde_json::from_str(&content) {
        Ok(todos) => todos,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return Err("Erreur lors du parsing du fichier JSON".to_string());
        }
    };

    if index < todos.len() {
        todos.remove(index);
    } else {
        return Err("Index invalide".to_string());
    }

    let updated_content = match serde_json::to_string_pretty(&todos) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing JSON: {}", e);
            return Err("Erreur lors de la sérialisation du JSON".to_string());
        }
    };

    if let Err(e) = std::fs::write("todo.json", updated_content) {
        eprintln!("Error writing file: {}", e);
        return Err("Erreur lors de l'écriture dans le fichier".to_string());
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_name_to_json,
            get_todos,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
