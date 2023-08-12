#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    pub pos: usize,
    pub line: usize,
    pub column: usize,
    pub line_pos: usize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            pos: 0,
            line: 0,
            column: 0,
            line_pos: 0,
        }
    }

    pub fn advance(&mut self, current_char: Option<char>) {
        self.pos += 1;
        self.column += 1;
        if let Some(hehe) = current_char {
            if hehe == '\n' {
                self.line += 1;
                self.column = 0;
                self.line_pos += 1;
            }
        }

    }
}