
pub enum EngineCommand {
    Insert { psotion: usize, text: String },
    Delete { start: usize, end: usize },
    Save { path: PathBuf },
    Load { path: PathBuf },
    GetMetrics
}

// Results returned to platform
pub enum EngineResult {
    Success,
    TextChanged { new_text: String },
    Metrics(EngineMetrics),
    Error(EngineError),
}

pub struct TextEngine {
    buffer_manager: BufferManager,
    file_system: FileSystem,
    lsp_client: LspClient,
    engine_metrics: EngineMetrics,
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

impl EnginMetrics {
    pub fn new() -> Self {

    }

    pub fn execute_command(&mut self, command: EngineCommand) -> EngineResult {

    }

    pub fn get_status(&self) -> EngineStatus {
        
    }
}