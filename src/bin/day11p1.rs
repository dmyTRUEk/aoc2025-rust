//! Advent of Code - Day 11 Part 1 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::read_to_string as read_file_to_string;

// use itertools::Itertools;
// use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

// use aoc2025_rust::utils::{};



fn main() {
	let solution = solve_file("./input/day11.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	let (adj_list, you) = parse_input(text);

	let mut adj_list: Vec<(Vec<u16>, u32)> = adj_list.into_iter()
		.map(|list| {
			// list.into_iter().map(|device_id| {
			// 	(device_id, if device_id != OUT { 0 } else { 1 })
			// }).collect()
			(list, 0)
		})
		.collect();

	// dbg!(&adj_list);

	fn back_flood(index: u16, adj_list: &mut Vec<(Vec<u16>, u32)>) -> u32 {
		if index == OUT {
			return 1
		}
		if adj_list[index as usize].1 != 0 {
			return adj_list[index as usize].1
		}
		let mut sum = 0;
		for i in adj_list[index as usize].clone().0.iter() {
			let n = back_flood(*i, adj_list);
			sum += n;
		}
		adj_list[index as usize].1 = sum;
		sum
	}

	let _ = back_flood(0, &mut adj_list);

	// dbg!(&adj_list);
	dbg!(&adj_list[you as usize]);

	adj_list[you as usize].1 as u64
}



fn parse_input(input: &str) -> (Vec<Vec<u16>>, u16) {
	let devices: Vec<(&str, &str)> = input.lines().map(|line| {
		let (self_name, output_devices) = line.split_once(": ").unwrap();
		(self_name, output_devices)
	}).collect();
	let mut you = None;
	let mut adj_list = vec![];
	for (i, (self_name, output_devices)) in devices.iter().enumerate() {
		let output_devices: Vec<u16> = output_devices
			.split(' ')
			.map(|od| {
				if od == "out" { OUT } else {
					devices.iter()
						.position(|d| d.0 == od).unwrap()
						.try_into().unwrap()
				}
			})
			.collect();
		adj_list.push(output_devices);
		if *self_name == "you" {
			assert!(you.is_none());
			let i: u16 = i.try_into().unwrap();
			you = Some(i);
		}
	}
	(adj_list, you.unwrap())
}

const OUT: u16 = u16::MAX;

// #[derive(Debug, Clone, Copy)]
// struct Device {
// 	name: [u8; 3],
// }
// impl Device {
// 	fn from_str(name: &str) -> Self {
// 		assert_eq!(3, name.len());
// 		let name = name.bytes().collect::<Vec<_>>().try_into().unwrap();
// 		Self { name }
// 	}
// }





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		5,
		solve_file("./input/day11.example")
	)
}

