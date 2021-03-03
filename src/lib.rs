use core::fmt;

mod binary_tree;
pub use binary_tree::generator;

pub trait MazeGenerator {
    fn generate(length:i32, width:i32) -> Maze;
}

pub struct Maze {
    data: Box<[Cell]>,
    start_index: usize,
    end_index: usize,
    length: i32,
    width: i32
}

struct Cell {
    north:bool,
    east:bool
}

impl Maze {
    fn new(data: Vec<Cell>, length: i32, width: i32, start_index: usize, end_index: usize) -> Maze {
        Maze {
            data: data.into_boxed_slice(),
            length,
            width,
            start_index,
            end_index
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut triggered = false;
        for i in (0..self.length as usize).rev() {
            let slice_start = i * self.width as usize;
            let contains_end = self.end_index > slice_start;
            let slice = &self.data[slice_start..slice_start+(self.width as usize)];
            for cell in slice {
                if !cell.north {
                    s.push_str("+---");
                }
                else {
                    s.push_str("+   ");
                }
            }
            s.push_str("+\r\n");

            if slice_start != self.start_index {
                s.push_str("|");
            }
            else {
                s.push_str(" ");
            }

            for cell in &slice[0..(slice.len()-1)] {
                if !cell.east {
                    s.push_str("   |");
                }
                else {
                    s.push_str("    ")
                }
            }

            let last_cell = slice.last().unwrap();
            if !last_cell.east  && (!contains_end || triggered){
                s.push_str("   |");
            }
            else {
                s.push_str("    ");
                if contains_end {
                    triggered = true;
                }
            }

            s.push_str("\r\n");
        }

        for _ in 0..self.width {
            s.push_str("+---");
        }
        s.push_str("+");

        write!(f, "{}", s)
    }
}