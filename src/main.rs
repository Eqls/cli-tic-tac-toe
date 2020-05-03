use std::convert::From;
use std::io;
use std::string::String;
use std::vec::Vec;

fn main() {
    let dims = get_grid_size();
    println!("{:?}", dims)
}

fn get_grid_size() -> Vec<i32> {
    println!("Please input your a grid size. For ex.: 3x3.");

    let mut grid_size = String::new();

    io::stdin()
        .read_line(&mut grid_size)
        .expect("Failed to read line");

    let tokens: Vec<i32> = grid_size
        .split("x")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    return tokens;
}
