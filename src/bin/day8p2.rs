//! Advent of Code - Day 8 Part 2 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::{collections::HashSet, fs::read_to_string as read_file_to_string};

// use itertools::Itertools;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};



fn main() {
	let solution = solve_file("./input/day8.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	let boxes = parse_input(text);
	let boxes_n = boxes.len();
	let dist2_matrix: Vec<Vec<u64>> = Vec::from_fn(boxes_n, |i| Vec::from_fn(boxes_n, |j| boxes[i].dist2_to(&boxes[j])));
	// connected[i] == [j,k,l] means that box[i] connected to box[j], box[k] and box[l]
	let mut connected: Vec<HashSet<u16>> = vec![HashSet::new(); boxes_n];
	let mut connection_i: u32 = 0;
	let mut last_connection: Option<(u16, u16)> = None;
	while connected.iter().map(|connections| connections.len()).max().unwrap() < boxes_n {
		println!("connection_i: {connection_i} {d}", d="-".repeat(10)); connection_i += 1;

		println!("notconnected_dist2s: BEGIN");
		let mut notconnected_dist2s: Vec<(u64, (u16, u16))> = vec![];
		for i in 0..boxes_n as u16 {
			for j in 0..i {
				debug_assert_eq!(dist2_matrix[i as usize][j as usize], dist2_matrix[j as usize][i as usize]);
				if !connected[i as usize].contains(&j) {
					notconnected_dist2s.push(( dist2_matrix[i as usize][j as usize], (i,j) ));
				}
			}
		}
		notconnected_dist2s.sort();
		// println!("{notconnected_dist2s:?}");
		let i = notconnected_dist2s[0].1.0;
		let j = notconnected_dist2s[0].1.1;
		// assert!(i < j);
		// println!("best connection({i} - {j}): {:?} - {:?}", boxes[i as usize], boxes[j as usize]);
		let true = connected[i as usize].insert(j) else { unreachable!() };
		let true = connected[j as usize].insert(i) else { unreachable!() };
		// println!("connected: {connected:?}");
		last_connection = Some((i, j));
		println!("notconnected_dist2s: END");

		println!("transitive connections: BEGIN");
		let mut transitive_connections_n: u32 = 0;
		loop { // make transitive connections happen!
			println!("transitive_connections_n: {transitive_connections_n}"); transitive_connections_n += 1;
			let mut is_changed = false;
			for i in 0..boxes_n as u16 {
				for j in connected[i as usize].clone() {
					if i == j { continue }
					for k in connected[j as usize].clone() {
						if !connected[i as usize].contains(&k) {
							// let true = connected[i as usize].insert(k) else { unreachable!() };
							let _ = connected[i as usize].insert(k);
							is_changed = true;
						}
					}
				}
				// connected[i].sort();
				// connected[i].dedup();
			}
			if !is_changed { break }
		}
		println!("transitive connections: END");
		// println!("connected: {connected:?}");
	}
	// println!();
	println!("connected: {connected:?}");
	connected.sort_by_key(|connections| connections.len());
	connected.dedup();
	connected.reverse();
	// println!("connected: {connected:?}");
	let (i, j) = last_connection.unwrap();
	(boxes[i as usize].x as u64) * (boxes[j as usize].x as u64)
}



fn parse_input(input: &str) -> Vec<Pos> {
	input.lines().map(|line| {
		let xyz = line.split(',').map(|n_str| {
			n_str.parse().unwrap()
		}).collect::<Vec<u32>>();
		let [x, y, z] = xyz.try_into().unwrap();
		Pos { x, y, z }
	}).collect()
}

#[derive(Debug)]
struct Pos {
	x: u32,
	y: u32,
	z: u32,
}
impl Pos {
	fn dist2_to(&self, other: &Self) -> u64 {
		((self.x as i32 - other.x as i32) as i64).pow(2) as u64 +
		((self.y as i32 - other.y as i32) as i64).pow(2) as u64 +
		((self.z as i32 - other.z as i32) as i64).pow(2) as u64
	}
}





trait VecFromFn<T> {
	fn from_fn<F: FnMut(usize) -> T>(len: usize, f: F) -> Self;
}
impl<T> VecFromFn<T> for Vec<T> {
	fn from_fn<F: FnMut(usize) -> T>(len: usize, f: F) -> Self {
		(0..len).map(f).collect()
	}
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		25272,
		solve_file("./input/day8.example")
	)
}

