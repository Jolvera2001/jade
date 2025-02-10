use serde::{Deserialize, Serialize};

#[tauri::command]
pub async fn update_content(range_offset: i32, range_length: i32, text: String, range: Range) -> Result<(), String> {
    println!("Recieved raw change: {:?}", range_offset);
    println!("Change at offset: {}, length: {}, new text: {}",
        range_offset,
        range_length,
        text
    );
    Ok(())
}

#[tauri::command]
pub async fn test_call(offset: i32) -> Result<(), String> {
    println!("{:?}", offset);
    Ok(())
    // somerhing
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "startLineNumber")]
    pub startline_number: i32,
    #[serde(rename = "startColumn")]
    pub start_column: i32,
    #[serde(rename = "endLineNumber")]
    pub endline_number: i32,
    #[serde(rename = "endColumn")]
    pub end_column: i32,
}