//! Advent of Code - Day 3 Part 2 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::fs::{read_to_string as read_file_to_string};

// use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};



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
	let mut res: u64 = 0;
	for battery in batteries {
		let mut n: u64 = 0;
		let mut index_of_prev_digit: Option<u8> = None;
		const DIGITS_N: u8 = 12;
		for digit_i in 0..DIGITS_N {
			// N=2
			// i=0 -> 0 .. l-1
			// i=1 -> iop+1 .. l
			//
			// N=3
			// i=0 -> 0 .. l-2
			// i=1 -> x .. l-1
			// i=2 -> x .. l-0
			let index_left = index_of_prev_digit.map(|i| i + 1).unwrap_or(0);
			// N=2
			// i=0 -> 10^1
			// i=1 -> 10^0
			//
			// N=3
			// i=0 -> 10^2
			// i=1 -> 10^1
			// i=2 -> 10^0
			let rindex_right = DIGITS_N - 1 - digit_i;
			let index_right = battery.len() as u8 - rindex_right;
			let iopd = index_left + (&battery[index_left as usize .. index_right as usize]).position_first_max().unwrap() as u8;
			index_of_prev_digit = Some(iopd);
			n += 10_u64.pow(rindex_right as u32) * (battery[iopd as usize] as u64);
		}
		dbg!(n);
		res += n;
	}
	res
}



fn parse_input(input: &str) -> Vec<Vec<u8>> {
	input.lines().map(|line| {
		line.bytes().map(|b| {
			b - b'0'
		}).collect()
	}).collect()
}



trait IndexOfMax<T> {
	fn position_first_max(&self) -> Option<u32>;
}
impl<T: PartialOrd> IndexOfMax<T> for &[T] {
	fn position_first_max(&self) -> Option<u32> {
		let mut option_index_of_max = None;
		for i in 0..self.len() as u32 {
			match option_index_of_max {
				None => {
					option_index_of_max = Some(i);
				}
				Some(index_of_max) if self[i as usize] > self[index_of_max as usize] => {
					option_index_of_max = Some(i);
				}
				_ => {}
			}
		}
		option_index_of_max
	}
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		3121910778619,
		solve_file("./input/day3.example")
	)
}

