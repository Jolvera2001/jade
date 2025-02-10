use std::process::{Command, Stdio, Child};
use std::sync::Mutex;

#[tauri::command]
async fn init_lsp(state: tauri::State<'_, Mutex<LspManager>>) -> Result<(), String> {
    let mut manager = state.lock().unwrap();
    manager.start_server()
}

#[tauri::command]
async fn send_lsp_message(message: String, state: tauri::State<'_, Mutex<LspManager>>) -> Result<String, String> {
    let manager = state.lock().unwrap();
    if let Some(process) = &manager.process {
        // Send message to LSP server and get response
    }
    Ok("response".to_string())
}
struct LspManager {
    process: Option<Child>
}

impl LspManager {
    fn new() -> Self {
        Self { process: None }
    }

    fn start_server(&mut self) -> Result<(), String> {
        let process = Command::new("rust-analyzer")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        self.process = Some(process);
        Ok(())
    }
}