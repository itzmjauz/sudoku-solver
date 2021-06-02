use std::env;

fn print_world(world: [[u8; 9]; 9]) {
    println!("Printing world:");
    for (world_row, y) in world.iter().zip(0..9) {
        if y == 3 || y == 6 {
            println!("{:-<21}", "");
        }
        for (c, x) in world_row.iter().zip(0..9) {
            print!("{} ", c);
            if x == 2 || x == 5 {
                print!("| ");
            }
        }
        print!("\n");
    }
}

fn parse_init_vector() -> Option<[u8; 81]> {
    let args: Vec<String> = env::args().collect();
    let mut init_vector: [u8; 81] = [0; 81];

    if args.len() != 2 || args[1].len() != 81 {
        println!("Args incorrect");
        println!("Defaulting to empty sudoku");
        return None;
    }

    for (i, c) in args[1].chars().enumerate() {
        let num = c as u8 - 0x30;
        if num > 9 {
            return None;
        }
        init_vector[i] = num;
    }
    Some(init_vector)
}

fn parse_world_vector(init_vector: [u8; 81]) -> [[u8; 9]; 9] {
    let mut world: [[u8; 9]; 9] = [[0; 9]; 9];

    for (world_row, i) in world.iter_mut().zip(0..9) {
        let init_vector_slice: &[u8] = &init_vector[(i * 9)..((i + 1) * 9)];

        for (world_location, init_vector_location) in
            world_row.iter_mut().zip(init_vector_slice.iter())
        {
            *world_location = *init_vector_location;
        }
    }
    world
}

fn create_world(init: Option<[u8; 81]>) -> [[u8; 9]; 9] {
    match init {
        None => [[0; 9]; 9],
        Some(init_vector) => parse_world_vector(init_vector),
    }
}

fn main() {
    let init_vector = parse_init_vector();
    let world = create_world(init_vector);
    print_world(world);
}
