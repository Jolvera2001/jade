use serde::{Deserialize, Serialize};

#[tauri::command]
pub fn update_content(changes: Vec<TextChange>) -> Result<(), String> {
    for change in changes {
        println!("Change at offset: {}, length: {}, new text: {}",
            change.range_offset,
            change.range_length,
            change.text
        );
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextChange {
    pub range_offset: usize,
    pub range_length: usize,
    pub text: String,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub start_line_number: u32,
    pub start_column: u32,
    pub end_line_number: u32,
    pub end_column: u32,
}