//! Advent of Code - Day 11 Part 2 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unreachable_patterns,
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
	let (adj_list, svr, fft, dac) = parse_input(text);

	let mut adj_list = adj_list.into_iter()
		.map(|list| {
			// list.into_iter().map(|device_id| {
			// 	(device_id, if device_id != OUT { 0 } else { 1 })
			// }).collect()
			(list, (false,false), (0,0,0,0))
		})
		.collect();

	// dbg!(&adj_list);

	fn back_flood(
		index: u16,
		adj_list: &mut Vec<(Vec<u16>, (bool,bool), (u32,u32,u32,u32))>,
		// svr: u16,
		fft: u16,
		dac: u16,
	) -> ((bool,bool), (u32,u32,u32,u32)) {
		if index == OUT {
			return ((false,false), (1,0,0,0))
		} else if index == fft {
			adj_list[index as usize].1.0 = true;
			if adj_list[index as usize].1.1 { // if dac
				adj_list[index as usize].2.3 = adj_list[index as usize].2.1;
			} else { // if not dac
				adj_list[index as usize].2.2 = adj_list[index as usize].2.0;
			}
		} else if index == dac {
			adj_list[index as usize].1.1 = true;
			if adj_list[index as usize].1.0 { // if fft
				adj_list[index as usize].2.3 = adj_list[index as usize].2.2;
			} else { // if not fft
				adj_list[index as usize].2.1 = adj_list[index as usize].2.0;
			}
		}
		{
			let n = match (adj_list[index as usize].1.0, adj_list[index as usize].1.1) {
				(false, false) => adj_list[index as usize].2.0, // - -
				(false, true) => adj_list[index as usize].2.1, // - dac
				(true, false) => adj_list[index as usize].2.2, // fft -
				(true, true) => adj_list[index as usize].2.3, // fft dac
			};
			if n != 0 {
				return (adj_list[index as usize].1, adj_list[index as usize].2)
			}
		}
		let mut sum = (0,0,0,0);
		for i in adj_list[index as usize].clone().0.iter() {
			let res = back_flood(*i, adj_list, /*svr,*/ fft, dac);
			sum.0 += res.1.0;
			sum.1 += res.1.1;
			sum.2 += res.1.2;
			sum.3 += res.1.3;
		}
		// adj_list[index as usize].2 = sum;
		(adj_list[index as usize].1, adj_list[index as usize].2)
	}

	let _ = back_flood(
		svr, // or 0?
		&mut adj_list,
		// svr,
		fft,
		dac
	);

	for line in adj_list.iter() {
		println!("{line:?}");
	}

	// dbg!(&adj_list);
	// dbg!(&adj_list[svr as usize]);

	adj_list[svr as usize].2.3 as u64
}



fn parse_input(input: &str) -> (Vec<Vec<u16>>, u16, u16, u16) {
	let devices: Vec<(&str, &str)> = input.lines().map(|line| {
		let (self_name, output_devices) = line.split_once(": ").unwrap();
		(self_name, output_devices)
	}).collect();
	let mut svr = None;
	let mut fft = None;
	let mut dac = None;
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
		match *self_name {
			"svr" => {
				assert!(svr.is_none());
				let i: u16 = i.try_into().unwrap();
				svr = Some(i);
			}
			"fft" => {
				assert!(fft.is_none());
				let i: u16 = i.try_into().unwrap();
				fft = Some(i);
			}
			"dac" => {
				assert!(dac.is_none());
				let i: u16 = i.try_into().unwrap();
				dac = Some(i);
			}
			_ => {}
		}
	}
	(adj_list, svr.unwrap(), fft.unwrap(), dac.unwrap())
}

const OUT: u16 = u16::MAX;





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		2,
		solve_file("./input/day11p2.example")
	)
}

