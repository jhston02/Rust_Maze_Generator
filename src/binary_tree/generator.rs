use crate::{MazeGenerator, Maze, Cell};
use rand::Rng;

pub struct BinaryTreeMazeGenerator;

impl MazeGenerator for BinaryTreeMazeGenerator {
    fn generate(length:i32, width:i32) -> Maze {
        let mut maze = generate_board(length, width);

        let mut rng = rand::thread_rng();

        //Do top row
        for w in 0..(width - 1) {
            let index = get_index_from_location(length - 1, w, width);
            let cell_option = maze.get_mut(index);
            if let Some(cell) = cell_option {
                cell.east = true;
            }
        }

        //Do last column
        for i in 0..(length - 1) {
            let index = get_index_from_location(i, width - 1, width);
            let cell_option = maze.get_mut(index);
            if let Some(cell) = cell_option {
                cell.north = true;
            }
        }

        for l in 0..(length - 1) {
            for w in 0..(width - 1) {
                let index = get_index_from_location(l, w, width);
                let cell_option = maze.get_mut(index);
                if let Some(cell) = cell_option {
                    if rng.gen_bool(0.5) {
                        cell.north = true;
                    }
                    else {
                        cell.east = true;
                    }
                }
            }
        }

        return Maze::new(maze, length, width, get_index_from_location(rng.gen_range(0..length), 0, width), get_index_from_location(rng.gen_range(0..length), width - 1, width));
    }
}

fn get_index_from_location(i:i32, w:i32, width:i32) -> usize {
    (w + (i * width)) as usize
}

fn generate_board(length:i32, width:i32) -> Vec<Cell> {
    let mut maze: Vec<Cell> = Vec::with_capacity((length * width) as usize);
    for _ in 0..length {
        for _ in 0..width {
            maze.push(Cell {
               north: false,
               east: false,
            });
        }
    }
    maze
}