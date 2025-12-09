//! Advent of Code - Day 9 Part 2 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::{collections::HashSet, fs::read_to_string as read_file_to_string};

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
	let red_tiles: Vec<TilePos> = parse_input(text);
	let red_or_green_tiles: HashSet<TilePos> = {
		let mut red_or_green_tiles = HashSet::new();
		let mut prev_tile = red_tiles.last().unwrap();
		for tile in red_tiles.iter() {
			if tile.x != prev_tile.x && tile.y == prev_tile.y {
				let y = tile.y;
				let x_min = tile.x.min(prev_tile.x);
				let x_max = tile.x.max(prev_tile.x);
				for x in x_min..=x_max { // x_min+1..=x_max-1
					let _ = red_or_green_tiles.insert(TilePos { x, y });
				}
			} else if tile.x == prev_tile.x && tile.y != prev_tile.y {
				let x = tile.x;
				let y_min = tile.y.min(prev_tile.y);
				let y_max = tile.y.max(prev_tile.y);
				for y in y_min..=y_max { // y_min+1..=y_max-1
					let _ = red_or_green_tiles.insert(TilePos { x, y });
				}
			} else {
				unreachable!()
			}
			prev_tile = tile;
		}
		red_or_green_tiles
	};
	let mut max_area: u64 = 0;
	for i in 0..red_tiles.len() {
		for j in 0..red_tiles.len() {
			if i == j { continue }
			let area = red_tiles[i].area_with(&red_tiles[j]);
			if area > max_area {
				let x_min = red_tiles[i].x.min(red_tiles[j].x);
				let x_max = red_tiles[i].x.max(red_tiles[j].x);
				let y_min = red_tiles[i].y.min(red_tiles[j].y);
				let y_max = red_tiles[i].y.max(red_tiles[j].y);
				let mut is_ok = true;
				'outer: for x in x_min..=x_max {
					for y in y_min..=y_max {
						if !red_or_green_tiles.contains(&TilePos { x, y }) {
							is_ok = false;
							break 'outer;
						}
					}
				}
				if is_ok {
					max_area = area;
				}
			}
		}
	}
	max_area
}



fn parse_input(input: &str) -> Vec<TilePos> {
	input.lines().map(|line| {
		let (x, y) = line.split_once(',').unwrap();
		let x = x.parse().unwrap();
		let y = y.parse().unwrap();
		TilePos { x, y }
	}).collect()
}

#[derive(Hash, PartialEq, Eq)]
struct TilePos {
	x: i32,
	y: i32,
}
impl TilePos {
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
		24,
		solve_file("./input/day9.example")
	)
}

