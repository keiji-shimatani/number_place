#[derive(Copy, Clone)]
pub struct CellInfo {
    posibility: i32,
}

impl CellInfo {
    pub fn create() -> CellInfo {
        CellInfo {
            posibility: ((1 << 10) - 2),
        }
    }

    pub fn create_by_number(i: u8) -> CellInfo {
        CellInfo {
            posibility:
                if 1 <= i && i <= 9 {
                    1 << i
                } else {
                    0
                }
        }
    }

    pub fn determined_number(&self) -> i32 {
        match self.posibility {
            2 => 1,
            4 => 2,
            8 => 3,
            16 => 4,
            32 => 5,
            64 => 6,
            128 => 7,
            256 => 8,
            512 => 9,
            _ => -1,
        }
    }

    pub fn is_confused(&self) -> bool { self.posibility == 0 }
    pub fn is_determined(&self) -> bool { self.determined_number() != -1 }
    pub fn is_possible(&self, n: u8) -> bool { (self.posibility & (1 << n)) != 0 }
    pub fn set_all_numbers(&mut self) { self.posibility = (1 << 10) - 2; }
    pub fn set_number(&mut self, n: u8) { self.posibility = 1 << n; }
    pub fn set_impossible(&mut self, n: u8) { self.posibility &= !(1 << n); }
    pub fn get_posibility(&self) -> i32 { self.posibility }
}

