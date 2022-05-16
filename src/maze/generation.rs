use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use rand::distributions::Uniform;
use rand::prelude::*;
use thiserror::Error;

/// A MazeCoordinate represents the coordinates of a single cell of the maze. Cells are laid
/// out in a grid based the number of rows and columns present. Cells are ephemeral and only kind of exist
/// based on the rows and columns said to be present in the maze. Cell coordinates are zero-based.
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct MazeCoordinate {
    pub row: i32,
    pub col: i32,
}

impl MazeCoordinate {
    pub fn random<T: Rng>(row_distribution: &Uniform<i32>, col_distribution: &Uniform<i32>, rng: &mut T) -> MazeCoordinate {
        let row = row_distribution.sample(rng);
        let col = col_distribution.sample(rng);

        MazeCoordinate { row, col }
    }

    /// Calculates the Manhattan Distance between this point and the other point.
    /// Guaranteed to be positive.
    pub fn manhattan_to(&self, other: &MazeCoordinate) -> i32 {
        (other.row - self.row).abs() + (other.col - self.col).abs()
    }

    pub fn moved(&self, row_change: i32, col_change: i32) -> MazeCoordinate {
        MazeCoordinate { row: self.row + row_change, col: self.col + col_change }
    }
}

#[cfg(test)]
mod maze_coordinate_tests {
    use super::MazeCoordinate;

    #[test]
    fn manhattan_distance_is_accurate() {
        let point = MazeCoordinate { row: 5, col: 8 };
        let point_ahead = MazeCoordinate { row: 10, col: 10 };
        let point_ahead_same_dist = MazeCoordinate { row: 8, col: 12 };
        let point_behind_same_dist = MazeCoordinate { row: 3, col: 3 };

        assert_eq!(7, point.manhattan_to(&point_ahead));
        assert_eq!(point.manhattan_to(&point_ahead), point.manhattan_to(&point_ahead_same_dist));
        assert_eq!(point.manhattan_to(&point_ahead), point.manhattan_to(&point_behind_same_dist));
    }
}

/// A MazeWall is a bidirectional edge between two cells in the maze representing an impassable wall.
/// It has a custom [PartialEq] and [Hash] implementation which makes
/// two MazeWalls equivalent regardless of the ordering of their coordinates.
/// As long as the two have the same starting and ending coordinates they are considered the same.
#[derive(Eq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct MazeWall {
    pub coord1: MazeCoordinate,
    pub coord2: MazeCoordinate,
}

impl PartialEq for MazeWall {
    fn eq(&self, other: &Self) -> bool {
        (self.coord1 == other.coord1 && self.coord2 == other.coord2) ||
            (self.coord1 == other.coord2 && self.coord2 == other.coord1)
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl Hash for MazeWall {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (lower_coord, higher_coord) = if self.coord1.row == self.coord2.row {
            if self.coord1.col < self.coord2.col {
                (&self.coord1, &self.coord2)
            } else {
                (&self.coord2, &self.coord1)
            }
        } else {
            if self.coord1.row < self.coord2.row {
                (&self.coord1, &self.coord2)
            } else {
                (&self.coord2, &self.coord1)
            }
        };

        lower_coord.hash(state);
        higher_coord.hash(state);
    }
}

#[cfg(test)]
mod maze_wall_tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::*;

    /// Creates a tuple of two equivalent [MazeWall]s which have their coordinates reversed
    fn create_inverse_tuple(coord1: MazeCoordinate, coord2: MazeCoordinate) -> (MazeWall, MazeWall) {
        (MazeWall { coord1, coord2 }, MazeWall { coord1: coord2, coord2: coord1 })
    }

    fn verify_equality_and_hashing(test_cases: &[(MazeWall, MazeWall)]) {
        test_cases.iter().for_each(| (first_wall, second_wall) | {
            println!("Verify: {:?} with {:?}", first_wall, second_wall);
            assert_eq!(first_wall, second_wall, "Equality: {:?} and {:?}", first_wall, second_wall);
            assert_eq!(second_wall, first_wall, "Equality: {:?} and {:?}", second_wall, first_wall);

            let mut first_wall_hasher = DefaultHasher::new();
            let mut second_wall_hasher = DefaultHasher::new();

            first_wall.hash(&mut first_wall_hasher);
            second_wall.hash(&mut second_wall_hasher);

            let first_wall_hash = first_wall_hasher.finish();
            let second_wall_hash = second_wall_hasher.finish();

            assert_eq!(first_wall_hash, second_wall_hash, "Hash comparison: {:?} and {:?}", first_wall, second_wall);
        });
    }

    #[test]
    fn equivalent_for_higher_row() {
        let compare_coord = MazeCoordinate { row: 10, col: 5 };
        let greater_col_coord = MazeCoordinate { row: 5, col: 6 };
        let lower_col_coord = MazeCoordinate { row: 5, col: 4 };
        let equal_col_coord = MazeCoordinate { row: 5, col: 5 };

        let test_cases = [
            create_inverse_tuple(compare_coord, greater_col_coord),
            create_inverse_tuple(compare_coord, lower_col_coord),
            create_inverse_tuple(compare_coord, equal_col_coord),
        ];

        verify_equality_and_hashing(&test_cases);
    }

    #[test]
    fn equivalent_for_lower_row() {
        let compare_coord = MazeCoordinate { row: 5, col: 5 };
        let greater_col_coord = MazeCoordinate { row: 10, col: 6 };
        let lower_col_coord = MazeCoordinate { row: 10, col: 4 };
        let equal_col_coord = MazeCoordinate { row: 10, col: 5 };

        let test_cases = [
            create_inverse_tuple(compare_coord, greater_col_coord),
            create_inverse_tuple(compare_coord, lower_col_coord),
            create_inverse_tuple(compare_coord, equal_col_coord),
        ];

        verify_equality_and_hashing(&test_cases);
    }

    #[test]
    fn equivalent_for_varying_column() {
        let compare_coord = MazeCoordinate { row: 10, col: 5 };
        let greater_coord = MazeCoordinate { row: 10, col: 6 };
        let lesser_coord = MazeCoordinate { row: 10, col: 4 };

        let test_cases = [
            create_inverse_tuple(compare_coord, compare_coord),
            create_inverse_tuple(compare_coord, greater_coord),
            create_inverse_tuple(compare_coord, lesser_coord),
        ];

        verify_equality_and_hashing(&test_cases);
    }
}

mod box_display {
    // Bars
    pub const BAR_HORIZ: &'static str = "─";
    pub const BAR_VERT: &'static str = "│";

    // Corners
    pub const CORNER_TL: &'static str = "┌";
    pub const CORNER_TR: &'static str = "┐";
    pub const CORNER_BL: &'static str = "└";
    pub const CORNER_BR: &'static str = "┘";

    // Tees
    pub const TEE_RIGHT: &'static str = "├";
    pub const TEE_LEFT: &'static str = "┤";
    pub const TEE_DOWN: &'static str = "┬";
    pub const TEE_UP: &'static str = "┴";

    // Quads
    pub const QUAD: &'static str = "┼";
}

pub struct Maze {
    start: MazeCoordinate,
    finish: MazeCoordinate,
    rows: i32,
    cols: i32,
    wall_edges: HashSet<MazeWall>
}


#[derive(Debug)]
pub enum MazeParam {
    Row,
    Col,
    PortalSpacing,
}

#[derive(Debug, Error)]
pub enum MazeConstructError {
    #[error("A parameter was invalid: {param:?} cannot be {value:?}")]
    ParameterInvalid { param: MazeParam, value: i32 },
    #[error("Maze was too small for the space between the portals (rows: {rows:?} cols: {cols:?} spacing: {portal_space:?}")]
    MazeTooSmallForSpacing { rows: i32, cols: i32, portal_space: i32 },
}

struct MazePortals {
    start: MazeCoordinate,
    end: MazeCoordinate,
}

impl Maze {
    pub fn start(&self) -> &MazeCoordinate {
        &self.start
    }
    pub fn finish(&self) -> &MazeCoordinate {
        &self.finish
    }
    pub fn rows(&self) -> i32 {
        self.rows
    }
    pub fn cols(&self) -> i32 {
        self.cols
    }
    pub fn wall_edges(&self) -> &HashSet<MazeWall> {
        &self.wall_edges
    }

    /// Asserts that a parameter for the Maze constructor is a positive value. Returns an error
    /// otherwise.
    fn assert_positive(param: MazeParam, value: i32) -> Result<(), MazeConstructError> {
        if value <= 0 {
            return Err(MazeConstructError::ParameterInvalid { param, value });
        }

        return Ok(())
    }

    fn in_maze(point: &MazeCoordinate, rows: i32, cols: i32) -> bool {
        (0..rows).contains(&point.row) && (0..cols).contains(&point.col)
    }

    /// Generate the initial set of walls in the maze where every cell in the grid is
    /// surrounded on all sides by walls
    fn generate_initial_walls(rows: i32, cols: i32) -> HashSet<MazeWall> {
        let mut wall_set: HashSet<MazeWall> = HashSet::new();

        for row in 0..(rows - 1) {
            for col in 0..(cols - 1) {
                let next_row = row + 1;
                let next_col = col + 1;

                wall_set.insert(MazeWall {
                    coord1: MazeCoordinate { row, col },
                    coord2: MazeCoordinate { row, col: next_col },
                });
                wall_set.insert(MazeWall {
                    coord1: MazeCoordinate { row, col },
                    coord2: MazeCoordinate { row: next_row, col },
                });
            }
        }

        // Last column
        for row in 0..(rows - 1) {
            let next_row = row + 1;
            let last_col = cols - 1;
            wall_set.insert(MazeWall {
                coord1: MazeCoordinate { row, col: last_col },
                coord2: MazeCoordinate { row: next_row, col: last_col },
            });
        }

        // Last row
        for col in 0..(cols - 1) {
            let next_col = col + 1;
            let last_row = rows - 1;
            wall_set.insert(MazeWall {
                coord1: MazeCoordinate { row: last_row, col },
                coord2: MazeCoordinate { row: last_row, col: next_col },
            });
        }
        return wall_set;
    }

    /// Selects the start and end cells for the maze. [portal_space] asserts the minimum required
    /// manhattan distance between the start and end portal.
    fn select_portal_coordinates(rows: i32, cols: i32, portal_space: i32) -> MazePortals {
        let mut shared_rng_engine = thread_rng();
        let row_distribution = Uniform::from(0..rows);
        let col_distribution = Uniform::from(0..cols);

        loop {
            let point1 = MazeCoordinate::random(&row_distribution, &col_distribution, &mut shared_rng_engine);
            let point2 = MazeCoordinate::random(&row_distribution, &col_distribution, &mut shared_rng_engine);
            let manhattan_distance = point1.manhattan_to(&point2);

            if manhattan_distance >= portal_space {
                return MazePortals { start: point1, end: point2 };
            }
        }
    }

    fn remove_walls_for_valid_maze(wall_set: &mut HashSet<MazeWall>, rows: i32, cols: i32, portals: &MazePortals) {
        // Do a flood fill starting from the start point
        // If we run out of options to move, remove a wall
        // If we hit the endpoint, it's a valid maze. Stop removing walls.
        let mut rng = thread_rng();

        loop {
            let mut move_space_queue: VecDeque<MazeCoordinate> = VecDeque::with_capacity((rows * cols / 2) as usize);
            let mut flooded_cells: HashSet<MazeCoordinate> = HashSet::with_capacity((rows * cols * 3 / 4) as usize);
            move_space_queue.push_front(portals.start);
            flooded_cells.insert(portals.start);

            while let Some(coordinate) = move_space_queue.pop_back() {
                // If we managed to flood from the start to the end it's a valid maze, return
                if coordinate == portals.end {
                    return
                }

                // Try to generate the next points to move to
                for row_direction in -1..=1 {
                    for col_direction in -1..=1 {
                        if (row_direction as i32 + col_direction).abs() != 1 {
                            continue
                        }

                        let new_coordinate = coordinate.moved(row_direction, col_direction);

                        // Can't use this coordinate if it's not in the maze or one of the cells we've already flooded
                        if !Self::in_maze(&new_coordinate, rows, cols) || flooded_cells.contains(&new_coordinate) {
                            continue
                        }
                        let intended_move = MazeWall { coord1: coordinate, coord2: new_coordinate };
                        // Can't move to the new space if there's a wall in the way
                        if wall_set.contains(&intended_move) {
                            continue
                        }

                        // This is a valid place we can move to, flood it and inspect later
                        move_space_queue.push_front(new_coordinate);
                        flooded_cells.insert(new_coordinate);
                    }
                }
            }

            // If we exhausted every space we can move to, we need to remove another wall
            // The only way this could return "none" is if all the walls have been removed,
            // and we must find a path through the maze before that happens
            let chosen_wall = wall_set.iter().choose(&mut rng).unwrap().clone();
            wall_set.remove(&chosen_wall);
        }
    }

    pub fn new(rows: i32, cols: i32, portal_space: i32) -> Result<Maze, MazeConstructError> {
        Self::assert_positive(MazeParam::Row, rows)?;
        Self::assert_positive(MazeParam::Col, cols)?;
        Self::assert_positive(MazeParam::PortalSpacing, portal_space)?;

        if rows + cols < portal_space {
            return Err(MazeConstructError::MazeTooSmallForSpacing { rows, cols, portal_space });
        }

        let mut initial_walls = Self::generate_initial_walls(rows, cols);
        let portals = Self::select_portal_coordinates(rows, cols, portal_space);
        Self::remove_walls_for_valid_maze(&mut initial_walls, rows, cols, &portals);

        return Ok(Maze {
            start: portals.start,
            finish: portals.end,
            rows,
            cols,
            wall_edges: initial_walls,
        });
    }
}

fn render_maze_top_or_bottom(maze: &Maze, row: i32, left_corner_str: &str, right_corner_str: &str, divide_tee: &str) -> String {
    let mut maze_border = String::from(left_corner_str);
    maze_border.push_str(box_display::BAR_HORIZ);

    for col in 1..maze.cols {
        let wall_to_test = MazeWall {
            coord1: MazeCoordinate { row, col: col - 1 },
            coord2: MazeCoordinate { row, col },
        };
        if maze.wall_edges.contains(&wall_to_test) {
            maze_border.push_str(divide_tee);
        } else {
            maze_border.push_str(box_display::BAR_HORIZ);
        }
        maze_border.push_str(box_display::BAR_HORIZ);
    }

    maze_border.push_str(right_corner_str);
    return maze_border;
}

fn render_maze_top(maze: &Maze) -> String {
    render_maze_top_or_bottom(maze, 0, box_display::CORNER_TL, box_display::CORNER_TR, box_display::TEE_DOWN)
}

fn render_maze_bottom(maze: &Maze) -> String {
    render_maze_top_or_bottom(maze, maze.rows - 1, box_display::CORNER_BL, box_display::CORNER_BR, box_display::TEE_UP)
}

fn render_maze_cell_content(maze: &Maze, row: i32) -> String {
    let mut cell_content = String::from(box_display::BAR_VERT);

    fn draw_cell_symbol(cell_content: &mut String, maze: &Maze, coordinate: &MazeCoordinate) {
        if coordinate.eq(&maze.start) {
            cell_content.push_str("S");
        } else if coordinate.eq(&maze.finish) {
            cell_content.push_str("F");
        } else {
            cell_content.push_str(" ");
        }
    }

    for col in 0..maze.cols {
        if col != maze.cols - 1 {
            let wall_test = MazeWall {
                coord1: MazeCoordinate { row, col },
                coord2: MazeCoordinate { row, col: col + 1 },
            };
            // Draw cell content
            draw_cell_symbol(&mut cell_content, &maze, &wall_test.coord1);

            // Draw next wall if it exists
            if maze.wall_edges.contains(&wall_test) {
                cell_content.push_str(box_display::BAR_VERT);
            } else {
                cell_content.push_str(" ");
            }
        } else {
            let coordinate = MazeCoordinate { row, col };
            draw_cell_symbol(&mut cell_content, &maze, &coordinate);
            cell_content.push_str(box_display::BAR_VERT);
        }
    }

    return cell_content;
}

fn render_maze_cell_divider(maze: &Maze, initial_row: i32) -> String {
    let mut divider = String::new();
    let initial_wall_test = MazeWall {
        coord1: MazeCoordinate { row: initial_row, col: 0 },
        coord2: MazeCoordinate { row: initial_row + 1, col: 0 },
    };

    if maze.wall_edges.contains(&initial_wall_test) {
        divider.push_str(box_display::TEE_RIGHT);
    } else {
        divider.push_str(box_display::BAR_VERT);
    }

    for col in 0..(maze.cols - 1) {
        let vert_wall_test = MazeWall {
            coord1: MazeCoordinate { row: initial_row, col },
            coord2: MazeCoordinate { row: initial_row + 1, col },
        };
        let vert_wall_test_next = MazeWall {
            coord1: MazeCoordinate { row: initial_row, col: col + 1 },
            coord2: MazeCoordinate { row: initial_row + 1, col: col + 1},
        };
        let horiz_wall_test_upper = MazeWall {
            coord1: MazeCoordinate { row: initial_row, col },
            coord2: MazeCoordinate { row: initial_row, col: col + 1 },
        };
        let horiz_wall_test_lower = MazeWall {
            coord1: MazeCoordinate { row: initial_row + 1, col },
            coord2: MazeCoordinate { row: initial_row + 1, col: col + 1 },
        };

        let left_vert_wall_exists = maze.wall_edges.contains(&vert_wall_test);
        let right_vert_wall_exists = maze.wall_edges.contains(&vert_wall_test_next);
        let top_horiz_wall_exists = maze.wall_edges.contains(&horiz_wall_test_upper);
        let bottom_horiz_wall_exists = maze.wall_edges.contains(&horiz_wall_test_lower);

        // Draw the vertical wall below the current cell if necessary
        if left_vert_wall_exists {
            divider.push_str(box_display::BAR_HORIZ);
        } else {
            divider.push_str(" ");
        }

        // This is gross and I really should've learned the unicode bits for generating box borders
        let corner_divide = match (left_vert_wall_exists, right_vert_wall_exists, top_horiz_wall_exists, bottom_horiz_wall_exists) {
            (true, true, true, true) => box_display::QUAD,
            (true, true, true, false) => box_display::TEE_UP,
            (true, true, false, true) => box_display::TEE_DOWN,
            (true, true, false, false) => box_display::BAR_HORIZ,
            (true, false, true, true) => box_display::TEE_LEFT,
            (true, false, true, false) => box_display::CORNER_BR,
            (true, false, false, true) => box_display::CORNER_TR,
            (false, true, true, true) => box_display::TEE_RIGHT,
            (false, true, true, false) => box_display::CORNER_BL,
            (false, true, false, true) => box_display::CORNER_TL,
            (false, false, true, true) => box_display::BAR_VERT,
            (true, false, false, false) |
                (false, true, false, false) |
                (false, false, true, false) |
                (false, false, false, true) |
                (false, false, false, false) => " ",
        };
        divider.push_str(corner_divide);
    }

    let last_wall_test = MazeWall {
        coord1: MazeCoordinate { row: initial_row, col: maze.cols - 1 },
        coord2: MazeCoordinate { row: initial_row + 1, col: maze.cols - 1 },
    };
    if maze.wall_edges.contains(&last_wall_test) {
        divider.push_str(box_display::BAR_HORIZ);
        divider.push_str(box_display::TEE_LEFT);
    } else {
        divider.push_str(" ");
        divider.push_str(box_display::BAR_VERT);
    }

    return divider;
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = Vec::with_capacity((self.rows * 2 + 1) as usize);
        lines.push(render_maze_top(self));

        for row in 0..self.rows {
            lines.push(render_maze_cell_content(self, row));
            if row == self.rows - 1 {
                lines.push(render_maze_bottom(self));
            } else {
                lines.push(render_maze_cell_divider(self, row));
            }
        }

        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod maze_tests {
    use crate::maze::generation::Maze;

    #[test]
    fn can_construct_maze() {
        let maze = Maze::new(25, 25, 10);
        assert!(maze.is_ok());
        let unwrapped_maze = maze.unwrap();
        println!(
            "{}\n Start: {:?}\n End: {:?}\n Manhattan Distance: {}",
             unwrapped_maze,
             unwrapped_maze.start,
             unwrapped_maze.finish,
             unwrapped_maze.start.manhattan_to(&unwrapped_maze.finish),
        );
    }
}