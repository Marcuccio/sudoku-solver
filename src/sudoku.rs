use std::fmt;

#[derive(Debug, Clone)]
pub enum CellLookup {
    Completed,
    // IllegalCell,
    LegalCell(usize,usize),
}

#[derive(Clone,Copy)]
pub struct Cell{
    value: u8,
    fixed: bool,
    candidates: [bool;9],
}

pub struct Sudoku{
    cells: [[Cell; 9];9],
    // single_candidates: Vec<(usize, usize)>,
}

impl Cell{
    fn new() -> Cell{
        Cell{
            value: 0,
            fixed: false,
            candidates:[true;9],
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    pub fn has_candidate(&self, v: u8) -> bool {
        self.candidates[(v-1) as usize]
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn update_candidate(&mut self, v: u8, b: bool) -> bool {
        self.candidates[(v-1) as usize] = b;
        if !b {
            self.validate_candidates()
        } else { true }
    }

    #[allow(dead_code)]
    pub fn get_candidates(&self) -> &[bool] {
        &self.candidates
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn validate_candidates(&self) -> bool {
        self.candidates.iter().fold(false, |acc, x| acc || *x)
    }
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku {cells: [[Cell::new();9];9],}
    }

    pub fn write_fixed(&mut self, v: u8 , r: usize, c: usize){
        self.cells[r][c].fixed = true;
        self.cells[r][c].value = v;

        self.update_candidates(v, r, c, false);
    }

    pub fn write(&mut self, v: u8, r: usize, c: usize) -> bool {
        debug_assert_eq!(self.cells[r][c].value, 0, "[{}][{}] value is not zero!\n", r,c);
        debug_assert_eq!(self.cells[r][c].has_candidate(v), true);

        self.cells[r][c].value = v;
        self.update_candidates(v, r, c, false)
    }

    pub fn erase(&mut self, r: usize, c: usize){
        debug_assert_ne!(self.cells[r][c].value, 0);
        debug_assert_eq!(self.cells[r][c].is_fixed(), false);

        let v = self.cells[r][c].value;
        self.cells[r][c].value = 0;
        // println!("erase: [{};{}]: {:?}", r, c, v);
        self.update_candidates(v, r, c, true);
        self.correct_candidates_for_value(v);
    }

    pub fn cell(&self, r: usize, c: usize) -> &Cell {
        &self.cells[r][c]
    }

    pub fn get_legal_empty_cell(&mut self) -> CellLookup {
        for r in 0..=8 {
            for c in 0..=8 {
                if self.cells[r][c].is_empty() && !self.cells[r][c].is_fixed() {
                    return CellLookup::LegalCell(r,c);
                }
            }
        }
        return CellLookup::Completed;
    }

    fn update_candidates(&mut self, v: u8, r: usize, c: usize, b:bool) -> bool {
        self.update_candidates_for_row(v, r, b) &&
        self.update_candidates_for_col(v, c, b) &&
        self.update_candidates_for_square(v, r, c, b)
    }

    fn update_candidates_for_row(&mut self, value: u8, r: usize, b:bool) -> bool {
        for c in 0..=8 {
            if !self.cells[r][c].is_fixed() && self.cells[r][c].is_empty() {
                if !self.cells[r][c].update_candidate(value, b) {
                    return false
                }
            }
        }
        true
    }

    fn update_candidates_for_col(&mut self, value: u8, c: usize, b:bool) -> bool{
        for r in 0..=8 {
            if !self.cells[r][c].is_fixed() && self.cells[r][c].is_empty() {
                if !self.cells[r][c].update_candidate(value, b) {
                    return false
                }
            }
        }
        true
    }

    fn correct_candidates_for_value(&mut self, v: u8){
        for r in 0..=8 {
            for c in 0..=8 {
                if self.cells[r][c].value == v {
                    self.update_candidates(v, r, c, false);
                }
            }
        }
    }

    fn update_candidates_for_square(&mut self, value: u8, r: usize, c: usize, b: bool) -> bool {
        let big_row = r/3*3;
        let big_col = c/3*3;

        let big_row_limit = big_row+3;
        let big_col_limit = big_col+3;

        for r in big_row..big_row_limit {
            for c in big_col..big_col_limit {
                if !self.cells[r][c].is_fixed() && self.cells[r][c].is_empty() {
                    if !self.cells[r][c].update_candidate(value, b) {
                        return false
                    }
                }
            }
        }
        true
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("");
        for (index, row) in self.cells.iter().enumerate(){
            for (c, cell) in row.iter().enumerate(){
                if (c+1)%3 == 0 && c < 8 {
                    //printf("asd");
                    //print!(" \x1B[1m{}\x1B[0m |",cell);
                    print!(" {} |",cell);
                }
                else{
                    print!(" {} ",cell);
                }
            }
            println!("");
            if (index+1)%3==0 && index < 8 {
                println!("-----------------------------");
            }
        }
        write!(f, "")
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.fixed {
             write!(f, "\x1B[1m{}\x1B[0m", self.value)
            //write!(f, "{:#b}{}\x1B[0m", BOLD, self.value)
        }
        else{
            //print!(" \x1B[1m{}\x1B[0m |",cell);
            write!(f, "\x1B[2m{}\x1B[0m", self.value)
        }
    }
}
