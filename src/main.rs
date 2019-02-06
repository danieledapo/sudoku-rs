use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let fp = env::args()
        .nth(1)
        .expect("please provide an input sudoku archive");

    let f = File::open(fp)?;
    let buf = BufReader::new(f);

    for (i, l) in buf.lines().enumerate() {
        let l = l?;

        let sudoku = sudoku_rs::Sudoku::from_line(&l).unwrap();

        let solution = sudoku.first_solution();
        let is_solved = solution.map_or(false, |s| s.is_solved());
        println!("#{} is solved {:?}", i, is_solved);

        if !is_solved {
            panic!();
        }
    }

    Ok(())
}
