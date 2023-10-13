struct SudokuGrid {
    grid: [[i32; 9]; 9]
}
impl SudokuGrid {
    fn display(&self) {
        for row in &self.grid {
            for &value in row {
                if value > 0 {
                    print!("|{value}")
                } else {
                    print!("| ")
                }
            }
            println!("|")
        }
    }

    fn exist_in_column(&self, search_value: i32, col: i32) -> bool {
        for row in &self.grid {
            let value = row[col];
            if value == search_value {
                return true;
            }
        }
        return false;
    }

    fn exist_in_row(&self, search_value: i32, row: i32) -> bool {
        for &value in &self.grid[row] {
            if value == search_value {
                return true;
            }
        }
        return false;
    }

}

fn main() {
    let example_grid = SudokuGrid {
        grid: [
            [0, 0, 0, 5, 9, 3, 4, 0, 6],
            [8, 9, 0, 4, 6, 7, 0, 0, 0],
            [6, 0, 0, 0, 0, 0, 0, 0, 7],
            [0, 0, 4, 9, 2, 0, 7, 0, 8],
            [0, 0, 1, 3, 0, 6, 9, 0, 0],
            [2, 0, 9, 0, 7, 4, 3, 0, 0],
            [3, 0, 0, 0, 0, 0, 0, 0, 4],
            [0, 0, 0, 7, 4, 8, 0, 3, 5],
            [4, 0, 2, 6, 3, 5, 0, 0, 0],
        ]
    };

    example_grid.display();
}
