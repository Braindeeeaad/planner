// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let return_str = format!("Hello, {}! You've been greeted from Rust!", name);
    return_str
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn add(left: u64, right: u64)->u64{
    right+left 
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*; 
    
    #[test] 
    fn exploration() {
        let result = add(2,2); 
        assert_eq!(result,4);
    }
}