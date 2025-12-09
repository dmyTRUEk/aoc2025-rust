//! Advent of Code - Day 3 Part 1 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::{read_to_string as read_file_to_string};

// use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

use aoc2025_rust::utils::PositionFirstMax;



fn main() {
	let solution = solve_file("./input/day3.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	let batteries = parse_input(text);
	println!("batteries: {batteries:?}");
	let mut res: u32 = 0;
	for battery in batteries {
		let index_of_first_digit = (&battery[..battery.len()-1]).position_first_max().unwrap();
		dbg!(index_of_first_digit);
		let index_of_second_digit = index_of_first_digit+1 + (&battery[(index_of_first_digit+1) as usize..]).position_first_max().unwrap();
		dbg!(index_of_second_digit);
		let first_digit = battery[index_of_first_digit as usize];
		let second_digit = battery[index_of_second_digit as usize];
		let n = 10*first_digit + second_digit;
		dbg!(n);
		res += n as u32;
	}
	res as u64
}



fn parse_input(input: &str) -> Vec<Vec<u8>> {
	input.lines().map(|line| {
		line.bytes().map(|b| {
			b - b'0'
		}).collect()
	}).collect()
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		357,
		solve_file("./input/day3.example")
	)
}

