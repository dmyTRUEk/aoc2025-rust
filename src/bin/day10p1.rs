//! Advent of Code - Day 10 Part 1 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::read_to_string as read_file_to_string;

// use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

use aoc2025_rust::utils::Bits;



fn main() {
	let solution = solve_file("./input/day10.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	let machines = parse_input(text);
	machines
		.into_iter()
		// .into_par_iter()
		.map(|machine| {
			let mut solutions_lens: Vec<u32> = vec![];
			for pattern in 0..2_u32.pow(machine.buttons.len() as u32) {
				// println!("machine = {machine:?}");
				// println!("{d} pattern = {pattern} = {pattern:b} {d}", d="-".repeat(10));
				let mut lights = vec![false; machine.lights.len()];
				for (i, pattern_i) in pattern.bits().into_iter().rev().enumerate() {
					// println!("i={i}\t pattern_i = {pattern_i}\t lights = {lights:?}");
					if pattern_i {
						for j in machine.buttons[/*32-*/i].iter() {
							lights[*j as usize] = !lights[*j as usize];
						}
					}
				}
				// println!("lights = {lights:?}");
				if lights == machine.lights {
					// println!("pattern = {pattern} = bits: {pattern:b} {d}", d="-".repeat(10));
					// println!("pattern.count_ones = {}", pattern.count_ones());
					solutions_lens.push(pattern.count_ones());
				}
			}
			let sol = solutions_lens.into_iter().min().unwrap();
			dbg!(sol);
			// println!("\n\n\n");
			sol
		})
		.sum::<u32>() as u64
}



fn parse_input(input: &str) -> Vec<Machine> {
	input.lines().map(|line| {
		let (lights, rest) = line.split_once(' ').unwrap();
		let (buttons, _joltage) = rest.rsplit_once(' ').unwrap();
		let lights: Vec<bool> = lights
			[1..lights.len()-1] // remove `[` and `]`
			.bytes()
			.map(|byte| match byte {
				b'.' => false,
				b'#' => true,
				_ => unreachable!("found byte for: `{}`", byte as char)
			})
			.collect();
		let buttons = buttons
			.split(' ')
			.map(|button| {
				let button = &button[1..button.len()-1]; // remove `(` and `)`
				let toggles_indices: Vec<u8> = button.split(',').map(|n| n.parse().unwrap()).collect();
				// let mut toggles = vec![false; lights.len()];
				// for i in toggles_indices {
				// 	assert!(!toggles[i as usize]);
				// 	toggles[i as usize] = true;
				// }
				// toggles
				toggles_indices
			})
			.collect();
		Machine { lights, buttons }
	}).collect()
}

#[derive(Debug)]
struct Machine {
	lights: Vec<bool>,
	buttons: Vec<Vec<u8>>,
	// _joltage: Vec<u16>,
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		7,
		solve_file("./input/day10.example")
	)
}

