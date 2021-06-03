use std::env;

#[derive(Clone, Copy, Debug)]
struct Cell {
    value: u8,
    options: [u8; 9],
}

struct World {
    world: [[Cell; 9]; 9],
}

impl World {
    fn init() -> World {
        World {
            world: [[Cell {
                value: 0,
                options: [1, 2, 3, 4, 5, 6, 7, 8, 9],
            }; 9]; 9],
        }
    }

    fn update_from_array(&mut self, arr: &[u8; 81]) {
        for y in 0..9 {
            for x in 0..9 {
                let value = arr[x + (9 * y) as usize];
                if value != 0 {
                    self.world[x][y].value = value;
                    self.world[x][y].options = [0, 0, 0, 0, 0, 0, 0, 0, 0];
                }
            }
        }
    }

    fn print(&self) {
        for y in 0..9 {
            for x in 0..9 {
                print!("{}", self.world[x][y].value);
                match x {
                    2 | 5 => print!(" | "),
                    _ => print!(" "),
                }
            }
            println!("");
            if y == 2 || y == 5 {
                println!("{:-<21}", "");
            }
        }
    }
}

fn parse_arg(arg: &str) -> Result<[u8; 81], &'static str> {
    let mut vector: [u8; 81] = [0; 81];

    for (i, c) in arg.chars().enumerate() {
        let num = c as u8 - 0x30;
        if num > 9 {
            return Err("Input numbers above 9 are not allowed:  0..9");
        }
        vector[i] = num;
    }

    return Ok(vector);
}

fn main() {
    //Collect arguments & init world
    let args: Vec<String> = env::args().collect();
    let mut world = World::init();
    //bound checking
    if args.len() > 1 {
        if args[1].len() != 81 {
            println!("Sudoku input string of incorrect length: {}", args[1].len());
        } else {
            world.update_from_array(&parse_arg(&args[1]).unwrap());
        }
    }
    //print world
    world.print();
    //world.solve_step();
}
