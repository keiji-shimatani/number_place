use crate::def;
use crate::cell;

#[derive(Copy, Clone)]
pub struct Info {
    pub cells: [[cell::CellInfo; def::COL_SIZE]; def::ROW_SIZE],
}

impl Info {
    pub fn create() -> Info {
        Info{
            cells: [[cell::CellInfo::create(); def::COL_SIZE]; def::ROW_SIZE],
        }
    }

    pub fn create_by_numbers(cells_: &def::Numbers) -> Info {
        let mut board_info = Info::create();
        board_info.set_cells_by_number(cells_);
        board_info
    }

    pub fn set_cells_by_number(&mut self, nums: &def::Numbers) {
        let mut pos = def::Position::create();
        while !pos.is_end() {
            let num = nums[pos.row][pos.col];
            if 1 <= num && num <= 9 {
                self.cells[pos.row][pos.col].set_number(num);
            }
            pos.next();
        }
    }

    pub fn get_cells(&self) -> [[cell::CellInfo; def::COL_SIZE]; def::ROW_SIZE] {
        self.cells
    }

    pub fn set_cells(&mut self, cells_: &[[cell::CellInfo; def::COL_SIZE]; def::ROW_SIZE]) {
        self.cells = *cells_;
    }

    pub fn is_determined(&self) -> bool {
        let mut pos = def::Position::create();
        while !pos.is_end() {
            if !self.cells[pos.row][pos.col].is_determined() {
                return false;
            }
            pos.next();
        }
        true
    }

    pub fn is_confused(&self) -> bool {
        let mut pos = def::Position::create();
        while !pos.is_end() {
            if self.cells[pos.row][pos.col].is_confused() {
                return true;
            }
            pos.next();
        }
        false
    }

    pub fn clear(&mut self) {
        let mut pos = def::Position::create();
        while !pos.is_end() {
            self.cells[pos.row][pos.col].set_all_numbers();
            pos.next();
        }
    }

    pub fn cell(&mut self, i: usize, j: usize) -> &mut cell::CellInfo {
        &mut self.cells[i][j]
    }

    pub fn determined_number(&self, i: usize, j: usize) -> i32 {
        self.cells[i][j].determined_number()
    }

    pub fn check_row(&mut self, pos: &def::Position) -> def::CheckResult {
        let mut is_changed = false;
        let mut j: usize = 0;
        while j < def::COL_SIZE {
            if j != pos.col {
                let n = self.cells[pos.row][j].determined_number();
                let target_cell = &mut self.cells[pos.row][pos.col];
                if n != -1 && target_cell.is_possible(n as u8) {
                    target_cell.set_impossible(n as u8);
                    is_changed = true;
                }
            }
            j += 1;
        }
        let target_cell = &self.cells[pos.row][pos.col];
        if target_cell.is_confused() {
            def::CheckResult::Confused
        } else if is_changed {
            def::CheckResult::Advanced
        } else {
            def::CheckResult::NoChange
        }
    }

    pub fn check_col(&mut self, pos: &def::Position) -> def::CheckResult {
        let mut is_changed = false;
        let mut i: usize = 0;
        while i < def::ROW_SIZE {
            if i != pos.row {
                let n = self.cells[i][pos.col].determined_number();
                let target_cell = &mut self.cells[pos.row][pos.col];
                if n != -1 && target_cell.is_possible(n as u8) {
                    target_cell.set_impossible(n as u8);
                    is_changed = true;
                }
            }
            i += 1;
        }
        let target_cell = &self.cells[pos.row][pos.col];
        if target_cell.is_confused() {
            def::CheckResult::Confused
        } else if is_changed {
            def::CheckResult::Advanced
        } else {
            def::CheckResult::NoChange
        }
    }

    pub fn check_squ(&mut self, pos: &def::Position) -> def::CheckResult {
        let mut is_changed = false;
        let rb = pos.row / 3 * 3;
        let cb = pos.col / 3 * 3;
        let mut i = rb;
        while i < rb + 3 {
            let mut j = cb;
            while j < cb + 3 {
                if i != pos.row || j != pos.col {
                    let n = self.cells[i][j].determined_number();
                    let target_cell = &mut self.cells[pos.row][pos.col];
                    if n != -1 && target_cell.is_possible(n as u8) {
                        target_cell.set_impossible(n as u8);
                        is_changed = true;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        let target_cell = &self.cells[pos.row][pos.col];
        if target_cell.is_confused() {
            def::CheckResult::Confused
        } else if is_changed {
            def::CheckResult::Advanced
        } else {
            def::CheckResult::NoChange
        }
    }
}

