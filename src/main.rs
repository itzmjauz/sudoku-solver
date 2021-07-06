#![allow(dead_code)]
#![allow(unused_imports)]
use std::env;

///The world struct, we want this to contain information on the world and what it contains
///But it should also describe the possible options a cell can take.
///A single option means we have found our solution for a given cell
#[derive(Debug)]
struct Sudoku {
    world: [[Cell; 9]; 9], //encode the world
    empty_cells: u8,       //track solved cells for deciding the solve
}
///The cell struct that encodes the sudoku world data
///We track a 'value' but also options, this might be redundant.
#[derive(Clone, Copy, Debug)]
struct Cell {
    value: u8,
    options: [bool; 9],
}

impl Sudoku {
    fn new(input: &str) -> Sudoku {
        //I wonder about the memory footprint here
        let mut world = Sudoku {
            world: [[Cell {
                value: 0,
                options: [true; 9],
            }; 9]; 9],
            empty_cells: 0,
        };

        for y in 0..9 {
            for x in 0..9 {
                let value = input.chars().nth(9 * y + x).unwrap().to_digit(10).unwrap() as u8;
                let ref mut cell = world.world[y][x];
                cell.value = value;
                // all options are false if a cell is populated; just a detail
                if cell.value != 0 {
                    cell.options = [false; 9];
                    world.empty_cells += 1;
                }
            }
        }

        world
    }
    //We reduce the data into a simple 2d struct, for printing
    fn world(self) -> [[u8; 9]; 9] {
        let mut world = [[0; 9]; 9];

        for y in 0..9 {
            for x in 0..9 {
                world[y][x] = self.world[y][x].value;
            }
        }
        world
    }
    // print world
    fn print(&self) {
        for row in self.world.iter() {
            for col in row.iter() {
                print!("{}, ", col.value);
            }
            println!("");
        }
    }
    // helper func
    fn print_options(&self) {
        for row in self.world.iter() {
            for col in row.iter() {
                println!("{:?}", col);
            }
        }
    }
    //propagate a cell's values
    fn propagate(&mut self, value: usize, x: usize, y: usize) {
        //println!("propagate() : value : {}, x : {}, y: {}", value, x, y);
        //set value
        self.world[y][x].value = value as u8;
        //row and col
        for i in 0..9 {
            self.world[y][i].options[value - 1] = false;
            self.world[i][x].options[value - 1] = false;
        }
        //block
        let bx = x / 3;
        let by = y / 3;
        for suby in 0..3 {
            for subx in 0..3 {
                //find the block
                let block_x = subx + (3 * bx);
                let block_y = suby + (3 * by);
                self.world[block_y][block_x].options[value - 1] = false;
            }
        }
        self.world[y][x].options[value - 1] = true;
    }
    //propagate all cell's values
    fn propagate_all(&mut self) {
        let mut empty_cells = 0;

        for y in 0..9 {
            for x in 0..9 {
                let cell = self.world[y][x];
                if cell.value != 0 {
                    self.propagate(cell.value as usize, x, y);
                } else {
                    empty_cells += 1;
                }
            }
        }

        self.empty_cells = empty_cells;
    }

    // find value with a single option
    fn solve_step_a(&self) -> Option<(usize, usize, usize)> {
        for (y, row) in self.world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.value == 0 {
                    let options: Vec<usize> = cell
                        .options
                        .iter()
                        .enumerate()
                        .filter(|&(_, e)| *e == true)
                        .map(|(i, _)| i + 1)
                        .collect();

                    if options.len() == 1 {
                        println!("unique value found!");
                        return Some((options[0], x, y));
                    }
                }
            }
        }
        return None;
    }
    // find value due to unique option
    fn solve_step_b(&self) -> Option<(usize, usize, usize)> {
        for value in 0..9 {
            for x in 0..9 {
                for y in 0..9 {
                    if self.world[y][x].value == 0 {
                        if self.unique_option_linear(value, x, y) {
                            return Some((value + 1, x, y));
                        } else if self.unique_option_block(value, x, y) {
                            return Some((value + 1, x, y));
                        }
                    }
                } 
            }
        }
        None
    }
    // unique in row + col
    fn unique_option_linear(&self, value: usize, x: usize, y: usize) -> bool {
        for i in 0..9 {
            if i != x && self.world[y][i].options[value] == true {
                return false;
            }
            if i != y && self.world[i][x].options[value] == true {
                return false;
            }
        }
        return true;
    }
    // unique in a block
    fn unique_option_block(&self, value: usize, x: usize, y: usize) -> bool {
        //println!("block start");
        let bx = x / 3;
        let by = y / 3;
        for subx in 0..3 {
            for suby in 0..3 {
                let block_x = subx + (bx * 3);
                let block_y = suby + (by * 3);

                if block_x != x
                    && block_y != y
                    && self.world[block_y][block_x].options[value] == true
                {
                    return false;
                }
                if self.world[block_y][block_x].value == value as u8 + 1 {
                    return false;
                }
                //println!("x : {}, y : {}, value: {}, option: {}", block_x, block_y, value, self.world[block_y][block_x].options[value]);
            }
        }
        //println!("unique value : {}, x: {}, y: {}", value, x, y);
        self.print();
        return true;
    }
}

fn main() {
    let mut sudoku = Sudoku::new(INPUT);
    sudoku.propagate_all();
    sudoku.print();
    while sudoku.empty_cells != 0 {
        if let Some((value, x, y)) = sudoku.solve_step_a() {
            println!("step_a : value : {} - x: {} - y: {}", value, x, y);
            sudoku.propagate(value, x, y);
            sudoku.empty_cells -= 1;
            continue;
        } else if let Some((value, x, y)) = sudoku.solve_step_b() {
            println!("step_b : value : {} - x: {} - y: {}", value, x, y);
            sudoku.propagate(value, x, y);
            sudoku.empty_cells -= 1;
            continue;
        } else {
            println!("cannot find an option...");
            break;
        }
    }
    sudoku.print();
    sudoku.print_options();
    
}

///test driven development
static INPUT: &str =
    "000820090500000000308040007100000040006402503000090010093004000004035200000700900";
static SUDOKU: [[u8; 9]; 9] = [
    [0, 0, 0, 8, 2, 0, 0, 9, 0],
    [5, 0, 0, 0, 0, 0, 0, 0, 0],
    [3, 0, 8, 0, 4, 0, 0, 0, 7],
    [1, 0, 0, 0, 0, 0, 0, 4, 0],
    [0, 0, 6, 4, 0, 2, 5, 0, 3],
    [0, 0, 0, 0, 9, 0, 0, 1, 0],
    [0, 9, 3, 0, 0, 4, 0, 0, 0],
    [0, 0, 4, 0, 3, 5, 2, 0, 0],
    [0, 0, 0, 7, 0, 0, 9, 0, 0],
];
// Test world parsing, we might want to use a special cell type though
#[test]
fn test_create_world() {
    let sudoku: Sudoku = Sudoku::new(INPUT);
    assert_eq!(sudoku.world(), SUDOKU);
}

#[test]
fn test_solve_step() {
    let sudoku: Sudoku = Sudoku::new(INPUT);
}
