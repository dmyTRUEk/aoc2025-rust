//! Advent of Code - Day 7 Part 1 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::{read_to_string as read_file_to_string};

// use rayon::iter::{IntoParallelIterator, ParallelIterator};



fn main() {
	dbg!(size_of_val(&b'x'));
	let solution = solve_file("./input/day7.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	use ManifoldCell::*;
	let mut manifold = parse_input(text);
	assert!(manifold.iter().all(|line| line.len() == manifold[0].len()));
	let mut split_n: u64 = 0;
	for y in 1..manifold.len() {
		let [line, line_above] = manifold.get_disjoint_mut([y, y-1]).unwrap();
		let line_above: &Vec<ManifoldCell> = line_above;
		assert_eq!(line.len(), line_above.len());
		for x in 0..line.len() {
			let cell_above = line_above[x];
			let cell = &mut line[x];
			if cell_above != Beam { continue }
			match cell {
				Empty => {
					*cell = Beam;
				}
				BeamSplitter => {
					line[x-1] = Beam;
					line[x+1] = Beam;
					split_n += 1;
				}
				Beam => {}
			}
		}
	}
	split_n
}



fn parse_input(input: &str) -> Vec<Vec<ManifoldCell>> {
	use ManifoldCell::*;
	input.lines().map(|line| -> Vec<ManifoldCell> {
		line.bytes().map(|byte| -> ManifoldCell {
			match byte {
				b'.' => Empty,
				b'^' => BeamSplitter,
				b'S' => Beam,
				_ => unreachable!("found byte for: `{}`", byte as char)
			}
		}).collect()
	}).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum ManifoldCell {
	Empty, BeamSplitter, Beam,
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		21,
		solve_file("./input/day7.example")
	)
}

