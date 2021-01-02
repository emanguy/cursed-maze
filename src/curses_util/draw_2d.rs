use std::cmp::{max, min, Ordering};

use ncurses::{chtype, mvaddch};

/// Represents a coordinate in screen space
#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32
}

impl Coordinate {
    /// Return a new coordinate shifted by (row_change, col_change)
    pub fn coord_shift(self, row_change: i32, col_change: i32) -> Coordinate {
        Coordinate { row: self.row + row_change, col: self.col + col_change }
    }
}

/// Draw a line on the screen with the following character
pub fn draw_line(from: Coordinate, to: Coordinate, fill_char: char) {
    let (from_lowcol, to_highcol) = if from.col < to.col {
        (&from, &to)
    } else {
        (&to, &from)
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
    for idx in 0..=col_change {
        let current_col = from_lowcol.col + idx;
        mvaddch(current_row, current_col, fill_char as chtype);
        total_row_change += row_change_per_col;
        let absolute_row_change = total_row_change.abs();

        // Move vertically if we moved down further than one pixel
        if absolute_row_change >= 1.0 {
            let mut rows_left_to_change = total_row_change as i32;
            let row_move = if row_change > 0 { 1 } else { -1 };

            while rows_left_to_change != 0 {
                rows_left_to_change = rows_left_to_change - row_move;
                mvaddch(current_row, current_col, '#' as chtype);
                current_row = current_row + row_move;
            }

            total_row_change -= ((total_row_change as i32) as f64);
        }
    }
}

#[derive(Debug)]
pub struct TriangleFillErr {
    part: Option<i8>,
    top_start: Coordinate,
    top_end: Coordinate,
    bottom_start: Coordinate,
    bottom_end: Coordinate,
    fill_err: RegionFillErr,
}

/// Fill a triangular region on the screen between 3 arbitrary points with the given fill character (fill_char)
pub fn fill_triangle(corner1: Coordinate, corner2: Coordinate, corner3: Coordinate, fill_char: char) -> Result<(), TriangleFillErr> {
    let mut sorted_corners = vec!(&corner1, &corner2, &corner3);
    sorted_corners.sort_by(|corner_a, corner_b| corner_a.col.cmp(&corner_b.col));
    let (corner_lowcol, corner_midcol, corner_highcol) = (sorted_corners[0], sorted_corners[1], sorted_corners[2]);

    // Draw lines if the coordinates are all in a line
    if corner_lowcol.row == corner_midcol.row && corner_lowcol.row == corner_highcol.row {
        draw_line(*corner_lowcol, *corner_highcol, fill_char);
        return Ok(());
    }
    if corner_lowcol.col == corner_midcol.col && corner_lowcol.col == corner_highcol.col {
        let lowest_row = min(min(corner_lowcol.row, corner_midcol.row), min(corner_lowcol.row, corner_highcol.row));
        let highest_row = max(max(corner_lowcol.row, corner_midcol.row), max(corner_lowcol.row, corner_highcol.row));

        draw_line(Coordinate { row: lowest_row, col: corner_lowcol.col }, Coordinate { row: highest_row, col: corner_lowcol.col }, fill_char);
        return Ok(());
    }

    let mapped_fill_region = |part: Option<i8>, top_start: &Coordinate, top_end: &Coordinate, bottom_start: &Coordinate, bottom_end: &Coordinate| {
        fill_region_between_lines(*top_start, *top_end, *bottom_start, *bottom_end, fill_char)
            .map_err(|err| TriangleFillErr {
                part,
                top_start: *top_start,
                top_end: *top_end,
                bottom_start: *bottom_start,
                bottom_end: *bottom_end,
                fill_err: err,
            })
    };

    // Handle cases where there is a vertical line on either side of the triangle
    if corner_lowcol.col == corner_midcol.col {
        let (top_start, bottom_start) = if corner_lowcol.row <= corner_midcol.row {
            (corner_lowcol, corner_midcol)
        } else {
            (corner_midcol, corner_lowcol)
        };
        let top_end = corner_highcol;
        let bottom_end = corner_highcol;

        mapped_fill_region(None, top_start, top_end, bottom_start, bottom_end)?;
        return Ok(());
    }
    if corner_midcol.col == corner_highcol.col {
        let top_start = corner_lowcol;
        let bottom_start = corner_lowcol;
        let (top_end, bottom_end) = if corner_midcol.row <= corner_highcol.row {
            (corner_midcol, corner_highcol)
        } else {
            (corner_highcol, corner_midcol)
        };

        mapped_fill_region(None, top_start, top_end, bottom_start, bottom_end)?;
        return Ok(());
    }

    // Otherwise break the triangle up into two regions and draw each
    let longline_slope = (corner_highcol.row - corner_lowcol.row) as f64 / (corner_highcol.col - corner_lowcol.col) as f64;
    let midpoint_distance = corner_midcol.col - corner_lowcol.col;
    let midpoint_rowchange = (longline_slope * midpoint_distance as f64) as i32;
    let second_midpoint = Coordinate { row: corner_lowcol.row + midpoint_rowchange, col: corner_midcol.col };

    // If the middle point is on the line between the low col corner and high col corner, just draw a line
    if second_midpoint.row == corner_midcol.row {
        draw_line(*corner_lowcol, *corner_highcol, fill_char);
        return Ok(());
    }

    // Classify the midpoints as the upper and lower ones
    let (upper_midpoint, lower_midpoint) = if second_midpoint.row <= corner_midcol.row {
        (&second_midpoint, corner_midcol)
    } else {
        (corner_midcol, &second_midpoint)
    };

    // Draw the 2 regions
    mapped_fill_region(Some(1), corner_lowcol, upper_midpoint, corner_lowcol, lower_midpoint)?;
    mapped_fill_region(Some(2), upper_midpoint, corner_highcol, lower_midpoint, corner_highcol)?;

    return Ok(());
}

#[derive(Debug)]
pub enum RegionFillErr {
    TopAndBottomDoNotAlign,
    TopIsBelowBottom,
}

/// Fill the area between 2 horizontal lines with the given fill character (fill_char)
fn fill_region_between_lines(top_line_start: Coordinate, top_line_end: Coordinate, bottom_line_start: Coordinate, bottom_line_end: Coordinate, fill_char: char) -> Result<(), RegionFillErr> {
    // Find leftmost points
    let (top_leftmost, top_rightmost) = if top_line_start.col > top_line_end.col {
        (&top_line_end, &top_line_start)
    } else {
        (&top_line_start, &top_line_end)
    };
    let (bottom_leftmost, bottom_rightmost) = if bottom_line_start.col > bottom_line_end.col {
        (&bottom_line_end, &bottom_line_start)
    } else {
        (&bottom_line_start, &bottom_line_end)
    };

    // Error checking
    if top_leftmost.col != bottom_leftmost.col || top_rightmost.col != bottom_rightmost.col {
        return Err(RegionFillErr::TopAndBottomDoNotAlign);
    }
    if top_leftmost.row > bottom_leftmost.row || top_rightmost.row > bottom_rightmost.row {
        return Err(RegionFillErr::TopIsBelowBottom);
    }

    // Set up variables for tracking the top and bottom lines
    let horiz_change = top_rightmost.col - top_leftmost.col;

    let top_vertchange = top_rightmost.row - top_leftmost.row;
    let top_vertchange_per_col = top_vertchange as f64 / horiz_change as f64;

    let bottom_vertchange = bottom_rightmost.row - bottom_leftmost.row;
    let bottom_vertchange_per_col = bottom_vertchange as f64 / horiz_change as f64;

    let mut top_total_row_change = 0.0;
    let mut bottom_total_row_change = 0.0;
    let mut top_row = top_leftmost.row;
    let mut bottom_row = bottom_leftmost.row;

    // Fill area between lines
    for idx in 0..=horiz_change {
        let col = top_leftmost.col + idx;
        for row in top_row..=bottom_row {
            mvaddch(row, col, fill_char as chtype);
        }

        top_total_row_change += top_vertchange_per_col;
        bottom_total_row_change += bottom_vertchange_per_col;
        let top_absolute_change = top_total_row_change.abs();
        let bottom_absolute_change = bottom_total_row_change.abs();

        if top_absolute_change > 1.0 {
            let num_changed_rows = top_total_row_change as i32;
            top_row += num_changed_rows;
            top_total_row_change -= num_changed_rows as f64;
        }
        if bottom_absolute_change > 1.0 {
            let num_changed_rows = bottom_total_row_change as i32;
            bottom_row += num_changed_rows;
            bottom_total_row_change -= num_changed_rows as f64;
        }
    }

    return Ok(());
}
