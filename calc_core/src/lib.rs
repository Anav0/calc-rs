const CELL_LIMIT: u16 = u16::MAX;
type CellLimitType = u16;

pub struct Sheet {
    pub values: Vec<String>,
    pub displayed: Vec<String>,
}
impl Sheet {
    pub fn new() -> Self {
        Self {
            values: vec![String::new(); CELL_LIMIT as usize],
            displayed: vec![String::new(); CELL_LIMIT as usize],
        }
    }
    pub fn change(self: &mut Self, x: u16, y: u16, value: &str, displayed: &str) {
        let pos = (x * y) + x;
        self.values[pos as usize] = String::from(value);
        self.displayed[pos as usize] = String::from(displayed);
    }
    pub fn change_s(self: &mut Self, x: u16, y: u16, value: &str) {
        let pos = (x * y) + x;
        self.values[pos as usize] = String::from(value);
        self.displayed[pos as usize] = String::from(value);
    }
}
