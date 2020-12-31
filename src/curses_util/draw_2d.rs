use std::cmp::{max, min};

use ncurses::{chtype, mvaddch};

#[derive(Copy, Clone)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32
}

impl Coordinate {
    pub fn coord_shift(self, row_change: i32, col_change: i32) -> Coordinate {
        Coordinate { row: self.row + row_change, col: self.col + col_change }
    }
}

pub fn draw_line(from: Coordinate, to: Coordinate, fill_char: char) {
    let (from_lowcol, to_highcol) = if from.col < to.col {
        (from, to)
    } else {
        (to, from)
    };

    let col_change = to_highcol.col - from_lowcol.col;
    let row_change = to_highcol.row - from_lowcol.row;

    // Edge case - vertical line
    if col_change == 0 {
        let lowest_row = min(from_lowcol.row, to_highcol.row);
        let highest_row = max(from_lowcol.row, to_highcol.row);

        for row in lowest_row..highest_row {
            mvaddch(row, from_lowcol.col, fill_char as chtype);
        }
        return;
    }

    let row_change_per_col = row_change as f64 / col_change as f64;

    let mut total_row_change: f64 = 0.0;
    let mut current_row = from_lowcol.row;

    // For each column, draw a pixel
    for idx in 0..col_change {
        let current_col = from_lowcol.col + idx;
        mvaddch(current_row, current_col, fill_char as chtype);
        total_row_change = total_row_change + row_change_per_col;
        let absolute_row_change = f64::abs(total_row_change);

        // Move vertically if we moved down further than one pixel
        if absolute_row_change >= 1.0 {
            let mut rows_left_to_change = total_row_change as i32;
            let row_move = if row_change > 0 { 1 } else { -1 };

            while rows_left_to_change != 0 {
                rows_left_to_change = rows_left_to_change - row_move;
                mvaddch(current_row, current_col, '#' as chtype);
                current_row = current_row + row_move;
            }

            total_row_change = total_row_change - ((total_row_change as i32) as f64);
        }
    }
}

pub fn fill_region(corner1: Coordinate, corner2: Coordinate, corner3: Coordinate, corner4: Coordinate) {

}
