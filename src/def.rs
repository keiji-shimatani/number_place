
pub const ROW_SIZE: usize = 9;
pub const COL_SIZE: usize = 9;
pub const BOARD_SIZE: usize = ROW_SIZE * COL_SIZE;
pub type Numbers = [[u8; COL_SIZE]; ROW_SIZE];
pub const NULL_CELLS: Numbers = [[0; COL_SIZE]; ROW_SIZE];
pub const DEFAULT_INIT_SEQUENCE: Numbers = [[1; COL_SIZE]; ROW_SIZE];

#[derive(PartialEq)]
pub enum CheckResult {
    Advanced,
    NoChange,
    Confused,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn create() -> Position {
        Position{
            row: 0,
            col: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.row = 0;
        self.col = 0;
    }

    pub fn next(&mut self) {
        if self.col < COL_SIZE - 1 {
            self.col += 1;
        } else {
            self.col = 0;
            self.row += 1;
        }
    }

    pub fn is_end(&self) -> bool {
        !(self.col < COL_SIZE && self.row < ROW_SIZE)
    }
}

