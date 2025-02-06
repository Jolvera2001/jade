use std::path::PathBuf;
use std::time::Duration;
use piecetable::PieceTable;

pub enum EngineCommand {
    Insert { postion: usize, text: String },
    Delete { start: usize, end: usize },
    Save { path: PathBuf },
    Load { path: PathBuf },
    GetMetrics
}

pub enum EngineError {
    FileNotFound,
    InvalidPosition,
    BufferOverflow,
    EncodingError,
    IoError(std::io::Error),  // Wrap standard IO errors
    LspError(String),         // Language server errors
    InvalidOperation(String), // For operations that aren't allowed in current state
}

pub enum EngineStatus {
    Ready,
    Loading,
    Saving,
    Processing,
    Error,
    ShuttingDown,
    // Could also include specific states:
    LspConnecting,
    IndexingFiles,
    WaitingForResponse,
}

// Results returned to platform
pub enum EngineResult {
    Success,
    TextChanged { new_text: String },
    Metrics(EngineMetrics),
    Error(EngineError),
}

struct BufferManager {
    current_buffer: PieceTable,  // or whatever buffer implementation
    encoding: TextEncoding,
    line_endings: LineEnding,
}

struct EngineMetrics {
    memory_usage: usize,
    operation_count: u64,
    last_operation_time: Duration,
    buffer_size: usize,
}

// main engine
pub struct TextEngine {
    buffer_manager: BufferManager,
    file_system: FileSystem,
    lsp_client: LspClient,
    engine_metrics: EngineMetrics,
}


impl EngineMetrics {
    pub fn new() -> Self {

    }

    pub fn execute_command(&mut self, command: EngineCommand) -> EngineResult {

    }

    pub fn get_status(&self) -> EngineStatus {
        
    }
}

#[tauri::command]
async fn execute_engine_command(command: EngineCommand) -> Result<EngineResult, String> {
    // Bridge between Tauri and engine
}