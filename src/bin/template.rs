//! Advent of Code - Day TODO Part TODO solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::read_to_string as read_file_to_string;

// use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

// use aoc2025_rust::utils::{};



fn main() {
	let solution = solve_file("./input/dayTODO.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	let TODO = parse_input(text);
	todo!()
}



fn parse_input(input: &str) -> (/*TODO*/) {
	todo!()
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		todo!("expected answer") as u64,
		solve_file("./input/dayTODO.example")
	)
}

