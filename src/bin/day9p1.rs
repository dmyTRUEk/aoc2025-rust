//! Advent of Code - Day 9 Part 1 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::{read_to_string as read_file_to_string};

// use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};



fn main() {
	let solution = solve_file("./input/day9.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	let tiles = parse_input(text);
	let mut max_area: u64 = 0;
	for i in 0..tiles.len() {
		for j in 0..tiles.len() {
			if i == j { continue }
			let area = tiles[i].area_with(&tiles[j]);
			if area > max_area {
				max_area = area;
			}
		}
	}
	max_area
}



fn parse_input(input: &str) -> Vec<RedTilePos> {
	input.lines().map(|line| {
		let (x, y) = line.split_once(',').unwrap();
		let x = x.parse().unwrap();
		let y = y.parse().unwrap();
		RedTilePos { x, y }
	}).collect()
}

struct RedTilePos {
	x: i32,
	y: i32,
}
impl RedTilePos {
	fn area_with(&self, other: &Self) -> u64 {
		let w = (self.x - other.x + 1).unsigned_abs() as u64;
		let h = (self.y - other.y + 1).unsigned_abs() as u64;
		w * h
	}
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		50,
		solve_file("./input/day9.example")
	)
}

