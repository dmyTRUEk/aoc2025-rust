//! Advent of Code - Day 10 Part 2 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::read_to_string as read_file_to_string;

// use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use aoc2025_rust::utils::Compositions;



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
	let machines_n = machines.len();
	machines
		// .into_iter()
		.into_par_iter()
		.enumerate()
		.map(|(machine_i, machine)| {
			let machine_n = machine_i + 1;
			println!("machine {machine_n}/{machines_n}: START SOLVING");
			let joltages_len = machine.joltages.len();
			let buttons_len = machine.buttons.len();
			let mut sol = None;
			'outer: for sol_len in 0..999_u32 {
				// itertools "cool" fns: [array_]combinations[_with_replacement], permutations, powerset
				'compositions: for buttons_presses in Compositions::new(buttons_len, sol_len as usize) {
					let mut joltajes = vec![0u16; joltages_len];
					for (i, button_presses) in buttons_presses.into_iter().enumerate() {
						for j in machine.buttons[i].iter() {
							joltajes[*j as usize] += button_presses as u16;
							if joltajes[*j as usize] >= machine.joltages[*j as usize] {
								continue 'compositions
							}
						}
					}
					if joltajes == machine.joltages {
						sol = Some(sol_len);
						break 'outer;
					}
				}
			}
			println!("machine {machine_n}/{machines_n}: SOLUTION = {sol:?}");
			sol.unwrap()
		})
		.sum::<u32>() as u64
}



fn parse_input(input: &str) -> Vec<Machine> {
	input.lines().map(|line| {
		let (_lights, rest) = line.split_once(' ').unwrap();
		let (buttons, joltages) = rest.rsplit_once(' ').unwrap();
		let buttons = buttons
			.split(' ')
			.map(|button| {
				let button = &button[1..button.len()-1]; // remove `(` and `)`
				let toggles_indices: Vec<u8> = button.split(',').map(|n| n.parse().unwrap()).collect();
				toggles_indices
			})
			.collect();
		let joltages = joltages
			[1..joltages.len()-1] // remove `{` and `}`
			.split(',')
			.map(|joltage| joltage.parse().unwrap())
			.collect();
		Machine { buttons, joltages }
	}).collect()
}

#[derive(Debug)]
struct Machine {
	// lights: Vec<bool>,
	buttons: Vec<Vec<u8>>,
	joltages: Vec<u16>,
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		33,
		solve_file("./input/day10.example")
	)
}

