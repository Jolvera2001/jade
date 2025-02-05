
#[derive(Copy, Clone)]
enum BufferType {
    Original,
    Added
}
struct Piece {
    buffer_type: BufferType,
    start: usize,
    length: usize,
}

pub struct PieceTable {
    original: String,
    added: String,
    pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(initial_text: String) -> Self {
        PieceTable {
            pieces: vec![Piece {
                buffer_type: BufferType::Original,
                start: 0,
                length: initial_text.len(),
            }],
            original: initial_text,
            added: String::new(),
        }
    }

    pub fn insert(&mut self, position: usize, text: &str) {
        let insert_start = self.added.len();
        self.added.push_str(text);

        let (piece_idx, piece_offset) = self.find_piece_at_position(position);

        if piece_offset > 0 {
            let current_piece = &self.pieces[piece_idx];

            // need to create three pieces (first part, new text, second part)
            let new_pieces = vec![
                Piece {
                    buffer_type: current_piece.buffer_type,
                    start: current_piece.start,
                    length: piece_offset,
                },
                Piece {
                    buffer_type: BufferType::Added,
                    start: insert_start,
                    length: text.len(),
                },
                Piece {
                    buffer_type: current_piece.buffer_type,
                    start: current_piece.start + piece_offset,
                    length: current_piece.length - piece_offset,
                },
            ];

            self.pieces.splice(piece_idx..piece_idx + 1, new_pieces);
        } else {
            self.pieces.insert(piece_idx, Piece {
                buffer_type: BufferType::Added,
                start: insert_start,
                length: text.len(),
            });
        }
    }

    pub fn delete(&mut self, start: usize, end: usize) {

    }

    pub fn get_text(&self) -> String {
        let mut result = String::new();

        for piece in &self.pieces {
            let text = match piece.buffer_type {
                BufferType::Original => &self.original[piece.start..piece.start + piece.length],
                BufferType::Added => &self.added[piece.start..piece.start + piece.length],
            };
            result.push_str(text);
        }

        result
    }

    pub fn get_text_range(&self, start: usize, end: usize) -> String {

    }

    fn find_piece_at_position(&self, position: usize) -> (usize, usize) {
        let mut current_pos = 0;
        for (idx, piece) in self.pieces.iter().enumerate() {
            if current_pos + piece.length > position {
                return (idx, position - current_pos);
            }
        }
        (self.pieces.len(), 0)
    }
}

