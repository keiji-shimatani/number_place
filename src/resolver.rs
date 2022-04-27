use crate::def;
use crate::board;

enum SearchResult {
    Continued,
    Finished,
    Confused
}

enum ResolverStatus {
    Start,
    Stop,
    Restart,
    Finish
}

pub struct Resolver {
    board_info: board::Info,
    init_cells: def::Numbers,
    restart_sequence: def::Numbers,
}

impl Resolver {
    pub fn create(cells: &def::Numbers, sequence: &def::Numbers) -> Resolver {
        Resolver {
            board_info: board::Info::create(),
            init_cells: *cells,
            restart_sequence: *sequence,
        }
    }

    fn check_row(&mut self, pos: &def::Position) -> def::CheckResult {
        self.board_info.check_row(pos)
    }

    fn check_col(&mut self, pos: &def::Position) -> def::CheckResult {
        self.board_info.check_col(pos)
    }

    fn check_squ(&mut self, pos: &def::Position) -> def::CheckResult {
        self.board_info.check_squ(pos)
    }

    fn search(&mut self, pos: &def::Position) -> bool {
        let orig_board_info = self.board_info.get_cells();
        let mut n = self.restart_sequence[pos.row][pos.col];
        let mut is_confused: bool;
        while n <= 9 {
            is_confused = false;
            self.board_info.set_cells(&orig_board_info);
            let mut is_advanced: bool;
            let target_cell = self.board_info.cell(pos.row, pos.col);
            if target_cell.is_possible(n) {
                target_cell.set_number(n);
                is_advanced = true;
                while is_advanced && !is_confused {
                    is_advanced = false;
                    let mut s_pos = def::Position::create();
                    while !s_pos.is_end() && !is_confused {
                        let mut stage = 0;
                        while stage < 3 && !is_confused {
                            let check_result = match stage {
                                0 => self.check_row(&s_pos),
                                1 => self.check_col(&s_pos),
                                2 => self.check_squ(&s_pos),
                                _ => def::CheckResult::Confused,
                            };
                            if check_result == def::CheckResult::Confused {
                                is_confused = true;
                            }
                            if check_result == def::CheckResult::Advanced {
                                is_advanced = true;
                            }
                            stage += 1;
                        }
                        s_pos.next();
                    }
                }
                if !is_confused {
                    if self.board_info.is_determined() {
                        self.restart_sequence[pos.row as usize][pos.col as usize] = n + 1;
                        return true;
                    }
                    self.restart_sequence[pos.row as usize][pos.col as usize] = n;
                    let mut next_pos = *pos;
                    next_pos.next();
                    if next_pos.is_end() || self.search(&next_pos) {
                        return true;
                    }
                }
            }
            n += 1;
        }
        self.restart_sequence[pos.row][pos.col] = 1;
        false
    }

    pub fn get_restart_sequence(&self) -> &def::Numbers {
        &self.restart_sequence
    }

    pub fn set_board(&mut self, nums: &def::Numbers) {
        self.init_cells = *nums;
    }

    pub fn set_restart_sequence(&mut self, nums: &def::Numbers) {
        self.restart_sequence = *nums;
    }

    pub fn initialize_restart_sequence(&mut self) {
        let mut i = 0;
        while i < def::ROW_SIZE {
            let mut j = 0;
            while j < def::COL_SIZE {
                self.restart_sequence[i][j] = 1;
                j += 1;
            }
            i += 1;
        }
    }

    pub fn restart(&mut self) -> bool {
        self.board_info.clear();
        self.board_info.set_cells_by_number(&self.init_cells);
        self.search(&def::Position::create())
    }

    pub fn start(&mut self) -> bool {
        self.initialize_restart_sequence();
        self.restart()
    }

    pub fn result(&self) -> Option<def::Numbers> {
        let mut nums = def::NULL_CELLS;
        let mut pos = def::Position::create();
        while !pos.is_end() {
            let n = self.board_info.cells[pos.row][pos.col].determined_number();
            if n == -1 {
                return None;
            }
            nums[pos.row][pos.col] = n as u8;
            pos.next();
        }
        Some(nums)
    }
}




