use crate::board::Board;

/// Portable Game Notation (PGN) is a standard plain text format for recording
/// chess games (both the moves and related data),
/// which can be read by humans and is also supported by most chess software.
///
struct PGNReader {
    board: Board,
}

impl PGNReader {
    pub fn new(board: Board) -> Self {
        Self { board }
    }

    pub fn advance(input: &mut str) {}
}
