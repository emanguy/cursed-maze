use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// A MazeCoordinate represents the coordinates of a single cell of the maze. Cells are laid
/// out in a grid based the number of rows and columns present. Cells are ephemeral and only kind of exist
/// based on the rows and columns said to be present in the maze. Cell coordinates are zero-based.
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct MazeCoordinate {
    row: i32,
    col: i32,
}

/// A MazeWall is a bidirectional edge between two cells in the maze representing an impassable wall.
/// It has a custom [PartialEq] and [Hash] implementation which makes
/// two MazeWalls equivalent regardless of the ordering of their coordinates.
/// As long as the two have the same starting and ending coordinates they are considered the same.
#[derive(Eq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct MazeWall {
    coord1: MazeCoordinate,
    coord2: MazeCoordinate,
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
    use super::{Maze, MazeCoordinate, MazeWall};

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

pub struct Maze {
    start: MazeCoordinate,
    finish: MazeCoordinate,
    rows: i32,
    cols: i32,
    wall_edges: HashSet<MazeCoordinate>
}
