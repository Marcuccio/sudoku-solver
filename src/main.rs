mod sudoku;

use sudoku::{Sudoku, CellLookup};

fn solve_sudoku(sudoku: &mut Sudoku)->bool {

    match sudoku.get_legal_empty_cell() {
        CellLookup::Completed => true,
        CellLookup::LegalCell(r,c) => {

            for num in 1..=9 {
                if sudoku.cell(r, c).has_candidate(num as u8) {
                    if sudoku.write(num as u8, r, c) {
                        if solve_sudoku(sudoku) {
                            return true;
                        }
                    }
                    sudoku.erase(r, c);
                }
            }
            false
        }
    }
}

fn main(){
    let mut sudoku = Sudoku::new();

    sudoku.write_fixed(7, 0, 2);
    sudoku.write_fixed(8, 0, 3);
    sudoku.write_fixed(9, 0, 8);

    sudoku.write_fixed(9, 1, 1);
    sudoku.write_fixed(1, 1, 4);
    sudoku.write_fixed(8, 1, 7);

    sudoku.write_fixed(5, 2, 0);
    sudoku.write_fixed(9, 2, 5);
    sudoku.write_fixed(7, 2, 6);

    sudoku.write_fixed(9, 3, 0);
    sudoku.write_fixed(3, 3, 5);
    sudoku.write_fixed(8, 3, 6);

    sudoku.write_fixed(4, 4, 1);
    sudoku.write_fixed(5, 4, 4);
    sudoku.write_fixed(9, 4, 7);

    sudoku.write_fixed(2, 5, 2);
    sudoku.write_fixed(4, 5, 3);
    sudoku.write_fixed(1, 5, 8);

    sudoku.write_fixed(4, 6, 2);
    sudoku.write_fixed(9, 6, 3);
    sudoku.write_fixed(3, 6, 8);

    sudoku.write_fixed(1, 7, 1);
    sudoku.write_fixed(2, 7, 7);
    sudoku.write_fixed(3, 7, 4);

    sudoku.write_fixed(6, 8, 0);
    sudoku.write_fixed(4, 8, 5);
    sudoku.write_fixed(5, 8, 6);

    println!("{}", sudoku);

    if solve_sudoku(&mut sudoku) {
        println!("Solved!\n{}", sudoku);
    } else {
        println!("Not solvable!");
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_update_row_candidates() {
        let mut sudoku = Sudoku::new();


        sudoku.write_fixed(4, 4, 1);
        sudoku.write_fixed(5, 4, 4);
        sudoku.write_fixed(9, 4, 7);

        let mut v = [true; 9];
        v[3] = false;
        v[4] = false;
        v[8] = false;

        assert_eq!(sudoku.cell(4, 0).get_candidates(), v);
        assert_eq!(sudoku.cell(4, 2).get_candidates(), v);
        assert_eq!(sudoku.cell(4, 3).get_candidates(), v);
        assert_eq!(sudoku.cell(4, 5).get_candidates(), v);
        assert_eq!(sudoku.cell(4, 6).get_candidates(), v);
        assert_eq!(sudoku.cell(4, 8).get_candidates(), v);
    }

    #[test]
    fn test_update_col_candidates() {
        let mut sudoku = Sudoku::new();

        sudoku.write_fixed(4, 1, 4);
        sudoku.write_fixed(5, 4, 4);
        sudoku.write_fixed(9, 7, 4);

        let mut v = [true; 9];
        v[3] = false;
        v[4] = false;
        v[8] = false;

        assert_eq!(sudoku.cell(0, 4).get_candidates(), v);
        assert_eq!(sudoku.cell(2, 4).get_candidates(), v);
        assert_eq!(sudoku.cell(3, 4).get_candidates(), v);
        assert_eq!(sudoku.cell(5, 4).get_candidates(), v);
        assert_eq!(sudoku.cell(6, 4).get_candidates(), v);
        assert_eq!(sudoku.cell(8, 4).get_candidates(), v);

    }

    #[test]
    fn test_update_block_candidates() {
        let mut sudoku = Sudoku::new();

        sudoku.write_fixed(7, 0, 2);
        sudoku.write_fixed(9, 1, 1);
        sudoku.write_fixed(5, 2, 0);

        let mut v = [true; 9];
        v[4] = false;
        v[6] = false;
        v[8] = false;

        assert_eq!(sudoku.cell(0, 0).get_candidates(), v);
        assert_eq!(sudoku.cell(0, 1).get_candidates(), v);
        assert_eq!(sudoku.cell(1, 0).get_candidates(), v);
        assert_eq!(sudoku.cell(1, 2).get_candidates(), v);
        assert_eq!(sudoku.cell(2, 1).get_candidates(), v);
        assert_eq!(sudoku.cell(2, 2).get_candidates(), v);
    }

    #[test]
    fn test_candidates_after_erase() {
        let mut sudoku = Sudoku::new();

        sudoku.write_fixed(9, 1, 1);
        sudoku.write(9, 8, 2);
        sudoku.erase(8,2);

        let mut v = [true; 9];

        assert_eq!(sudoku.cell(3, 0).get_candidates(), v);
        assert_eq!(sudoku.cell(4, 0).get_candidates(), v);
        assert_eq!(sudoku.cell(5, 0).get_candidates(), v);
        assert_eq!(sudoku.cell(6, 2).get_candidates(), v);
        assert_eq!(sudoku.cell(7, 2).get_candidates(), v);
        assert_eq!(sudoku.cell(8, 2).get_candidates(), v);

        v[8] = false;

        assert_eq!(sudoku.cell(6, 1).get_candidates(), v);
        assert_eq!(sudoku.cell(7, 1).get_candidates(), v);
        assert_eq!(sudoku.cell(8, 1).get_candidates(), v);

    }
}
