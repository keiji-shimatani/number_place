use std::io;
use std::env;

mod def;
mod cell;
mod board;
mod resolver;

fn convert_to_bytes(s: String) -> Option<def::Numbers> {
    //let bytes = s.into_bytes();
    let bytes = s.as_bytes();
    if bytes.len() != def::BOARD_SIZE {
        return None;
    }
    let mut nums = def::NULL_CELLS;
    let mut pos = def::Position::create();
    while !pos.is_end() {
        let n = bytes[pos.row * def::COL_SIZE + pos.col] - 0x30; // '0' = 0x30
        if n < 0 || n > 9 {
            return None;
        }
        nums[pos.row][pos.col] = n;
        pos.next();
    }
    Some(nums)
}

fn convert_to_cells(s: String) -> Option<def::Numbers> {
    convert_to_bytes(s)
}

fn convert_to_sequence(s: String) -> Option<def::Numbers> {
    match convert_to_bytes(s) {
        Some(mut nums) => {
            let mut pos = def::Position::create();
            while !pos.is_end() {
                let n = nums[pos.row][pos.col]; // '0' = 0x30
                nums[pos.row][pos.col] = if n == 0 {
                    10
                } else {
                    n
                };
                pos.next();
            }
            Some(nums)
        }
        None => None,
    }
}

fn convert_to_string(nums: &def::Numbers) -> Option<String> {
    let mut s = String::from("");
    let mut pos = def::Position::create();
    while !pos.is_end() {
        s.push((nums[pos.row][pos.col] + 0x30) as char);
        pos.next();
    }
    Some(s)
}

fn run_app() -> Result<(), i32> {
    /*
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        println!();

    }
    */
    let args: Vec<String> = env::args().collect();
    //println!("argc: {}", args.capacity());
    if args.len() != 2 && args.len() != 3 {
        return Err(1);
    }

    let initial_cells = match convert_to_cells(args[1].clone()) {
        None => { return Err(2); },
        Some(cells_) => cells_,
    };

    let initial_sequence: def::Numbers;
    initial_sequence = if args.len() == 3 {
        match convert_to_sequence(args[2].clone()) {
            None => { return Err(3); },
            Some(sequence_) => sequence_,
        }
    } else {
        def::DEFAULT_INIT_SEQUENCE
    };

    let mut res = resolver::Resolver::create(&initial_cells, &initial_sequence);

    let is_ok = res.restart();
    let result = res.result();
    let cells = convert_to_string(&result.unwrap_or(def::NULL_CELLS)).unwrap();
    let sequence = convert_to_string(
        & (if !is_ok || result.is_none() {
            def::NULL_CELLS
        } else {
            *res.get_restart_sequence()
        }
        )).unwrap();
    println!("{}", cells);
    println!("{}", sequence);

    /*
    for row in initial_cells.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
    */

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            //eprintln!("error: {:?}", err);
            //1
            err
        }
    });
}
