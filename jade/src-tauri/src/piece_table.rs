use serde::{Deserialize, Serialize};

#[tauri::command]
pub fn update_content(change: ModelContentChange) -> Result<(), String> {
    println!("Change at offset: {}, length: {}, new text: {}",
        change.range_offset,
        change.range_length,
        change.text
    );
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelContentChange {
    pub range_offset: i32,
    pub range_length: i32,
    pub text: String,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "startLineNumber")]
    pub start_line_number: i32,
    #[serde(rename = "startColumn")]
    pub start_column: i32,
    #[serde(rename = "endLineNumber")]
    pub end_line_number: i32,
    #[serde(rename = "endColumn")]
    pub end_column: i32,
}