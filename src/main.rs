use std::env;

struct World {
  world: [[u8; 9]; 9],
}

impl World {
  fn init(&mut self) {} //can maybe load in a world

  fn init_from_array(&mut self, arr: [u8; 81]) {
    for (world_row, i) in self.world.iter_mut().zip(0..9) {
      let arr_slice: &[u8] = &arr[(i * 9)..((i + 1) * 9)];

      for (world_location, arr_location) in world_row.iter_mut().zip(arr_slice) {
        *world_location = *arr_location;
      }
    }
  }

  fn print(self) {
    println!("Printing world..");
    for (world_row, y) in self.world.iter().zip(0..9) {
      for (world_col, x) in world_row.iter().zip(0..9) {
        print!("{}", world_col);
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
  let mut world = World { world: [[0; 9]; 9] };
  //bound checking
  if args.len() > 1 {
    if args[1].len() != 81 {
      println!("Sudoku input string of incorrect length: {}", args[1].len());
      world.init();
    } else {
      world.init_from_array(parse_arg(&args[1]).unwrap());
    }
  }
  //print world
  world.print();
}
