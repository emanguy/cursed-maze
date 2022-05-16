use crate::maze::generation::{Maze, MazeCoordinate, MazeWall};
use crate::{Pillar, Wall};

pub fn create_pillars_for_maze(maze: &Maze) -> Vec<Vec<Pillar>> {
    let mut pillar_vec = Vec::with_capacity((maze.rows() + 1) as usize);
    for row in 0..=maze.rows() {
        let mut row_vec = Vec::with_capacity((maze.cols() + 1) as usize);
        for col in 0..=maze.cols() {
            row_vec.push(Pillar::at((col * 4) as f64, (row * 4) as f64));
        }

        pillar_vec.push(row_vec);
    }

    return pillar_vec;
}

pub fn create_walls_for_maze<'pillar>(
    maze: &Maze,
    pillars: &'pillar Vec<Vec<Pillar>>,
) -> Vec<Wall<'pillar, 'pillar>> {
    let mut walls = Vec::new();
    // First, create the walls at the edge of the maze. They will never be removed.

    // Top and bottom rows
    for col in 0..maze.cols() {
        walls.push(Wall::from_pillars(
            &pillars[0][col as usize],
            &pillars[0][(col + 1) as usize],
        ));
        walls.push(Wall::from_pillars(
            &pillars[maze.rows() as usize][col as usize],
            &pillars[maze.rows() as usize][(col + 1) as usize],
        ));
    }

    // Left and right sides
    for row in 0..maze.rows() {
        walls.push(Wall::from_pillars(
            &pillars[row as usize][0],
            &pillars[(row + 1) as usize][0],
        ));
        walls.push(Wall::from_pillars(
            &pillars[row as usize][maze.cols() as usize],
            &pillars[(row + 1) as usize][maze.cols() as usize],
        ));
    }

    // Next, create the inner walls based on the wall set
    for row in 0..maze.rows() {
        for col in 0..maze.cols() {
            // Add a wall if there's a wall between this cell and the next one in the same row
            if maze.wall_edges().contains(&MazeWall {
                coord1: MazeCoordinate { row, col },
                coord2: MazeCoordinate { row, col: col + 1 },
            }) {
                walls.push(Wall::from_pillars(
                    &pillars[row as usize][(col + 1) as usize],
                    &pillars[(row + 1) as usize][(col + 1) as usize],
                ));
            }

            // Check to see if there's a wall between this cell and the next one in the same column
            if maze.wall_edges().contains(&MazeWall {
                coord1: MazeCoordinate { row, col },
                coord2: MazeCoordinate { row: row + 1, col },
            }) {
                walls.push(Wall::from_pillars(
                    &pillars[(row + 1) as usize][col as usize],
                    &pillars[(row + 1) as usize][(col + 1) as usize],
                ));
            }
        }
    }

    return walls;
}
