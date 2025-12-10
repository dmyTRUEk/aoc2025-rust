//! Advent of Code - Day 9 Part 2 solution.
//! $ cargo run --bin dayXpY

#![deny(
	unused_assignments,
	unused_results,
)]

use std::{collections::HashSet, fs::read_to_string as read_file_to_string};

use itertools::Itertools;
use rand::{rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};



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
	let red_tiles_n = red_tiles.len();
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
	let x_max_global = red_tiles.iter().map(|t| t.x).max().unwrap();
	let y_max_global = red_tiles.iter().map(|t| t.y).max().unwrap();
	let effective_x_max: Vec<u32> = {
		let mut v = vec![];
		for y in 0..=y_max_global+1 {
			dbg!(y);
			let mut x_max = 0;
			for x in 0..=x_max_global+1 {
				if red_or_green_tiles.contains(&TilePos { x, y }) {
					x_max = x;
				}
			}
			v.push(x_max);
		}
		v
	};

	// // let mut rng = rng();
	// let mut max_area: u64 = 0;
	// for i in 0..red_tiles_n {
	// 	// let i = rng.random_range(0..red_tiles_n);
	// 	// println!("ij_max={ij_max}\ti={i}\tj:", ij_max=red_tiles_n);
	// 	dbg!(max_area);
	// 	let i_max_area = (0..red_tiles_n)
	// 		.into_par_iter()
	// 		.filter_map(|j| {
	// 			let mut rng = rng(); // FIXME: REMOVE
	// 			let i = rng.random_range(0..red_tiles_n); // FIXME: REMOVE
	// 			let j = rng.random_range(0..red_tiles_n); // FIXME: REMOVE
	// 			if i == j { return None }
	// 			// println!("ij_max = {ij_max}\ti = {i}\tj = {j}\tmax_area = {max_area}", ij_max=red_tiles_n);
	// 			let area = red_tiles[i].area_with(&red_tiles[j]);
	// 			if area <= max_area { return None }
	// 			fn is_on_perimeter(tile_pos: &TilePos, perimeter: &HashSet<TilePos>) -> bool {
	// 				perimeter.contains(tile_pos)
	// 			}
	// 			fn is_inside_perimeter(t: TilePos, x_max: u32, perimeter: &HashSet<TilePos>) -> bool {
	// 				let mut n: u32 = 0;
	// 				for x in t.x..=x_max+1 {
	// 					if perimeter.contains(&TilePos { x, y: t.y }) && perimeter.contains(&TilePos { x, y: t.y+1 }) && perimeter.contains(&TilePos { x, y: t.y-1 }) {
	// 						n += 1;
	// 					}
	// 				}
	// 				n % 2 == 1
	// 			}
	// 			let x_min = red_tiles[i].x.min(red_tiles[j].x);
	// 			let x_max = red_tiles[i].x.max(red_tiles[j].x);
	// 			let y_min = red_tiles[i].y.min(red_tiles[j].y);
	// 			let y_max = red_tiles[i].y.max(red_tiles[j].y);
	// 			let mut is_ok = true;
	// 			/*if is_ok*/ {
	// 				let x = x_min;
	// 				for y in y_min..=y_max {
	// 					let tile_pos = TilePos { x, y };
	// 					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
	// 						is_ok = false;
	// 						break
	// 					}
	// 				}
	// 			}
	// 			if is_ok {
	// 				let x = x_max;
	// 				for y in y_min..=y_max {
	// 					let tile_pos = TilePos { x, y };
	// 					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
	// 						is_ok = false;
	// 						break
	// 					}
	// 				}
	// 			}
	// 			if is_ok {
	// 				let y = y_min;
	// 				for x in x_min..=x_max {
	// 					let tile_pos = TilePos { x, y };
	// 					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
	// 						is_ok = false;
	// 						break
	// 					}
	// 				}
	// 			}
	// 			if is_ok {
	// 				let y = y_max;
	// 				for x in x_min..=x_max {
	// 					let tile_pos = TilePos { x, y };
	// 					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
	// 						is_ok = false;
	// 						break
	// 					}
	// 				}
	// 			}
	// 			is_ok.then_some(area)
	// 		})
	// 		.max()
	// 	;
	// 	if let Some(i_max_area) = i_max_area && i_max_area > max_area {
	// 		max_area = i_max_area;
	// 	}
	// }
	// max_area



	// WRONG AREA FORMULA
	// const MAX_AREA: u64 = 1_303_549_718;
	// const MAX_AREA: u64 = 1_309_871_757;
	// const MAX_AREA: u64 = 1_518_085_800; // NOTE(answer): TOO LOW
	// const MAX_AREA: u64 = 1_525_957_356; // NOTE(answer): TOO LOW
	// const MAX_AREA: u64 = 1_616_804_724; // THIS IS NOT INSIDE // NOTE(answer): not right?
	// const MAX_AREA: u64 = 1_618_003_218; // THIS IS NOT INSIDE // NOTE(answer): TOO HIGH

	// CORRECT AREA FORMULA
	// const MAX_AREA: u64 = 1_310_082_477;
	const MAX_AREA: u64 = 1_525_991_432; // max from vis

	let max_area = (0..red_tiles_n).cartesian_product(0..red_tiles_n)
		.par_bridge()
		.filter_map(|(i, j)| {
			// print!(" {j}"); flush();
			if i == j { return None }
			let area = red_tiles[i].area_with(&red_tiles[j]);
			if area <= MAX_AREA-1 { return None } // -1 too not get none if `MAX_AREA` is really MAX area
			// if i < 247 && j > 247 { return None } // TODO: is this correct?
			// if j < 247 && i > 247 { return None } // TODO: is this correct?
			println!("ij_max = {ij_max}\ti = {i}\tj = {j}", ij_max=red_tiles_n);
			fn is_on_perimeter(tile_pos: &TilePos, perimeter: &HashSet<TilePos>) -> bool {
				perimeter.contains(tile_pos)
			}
			fn is_inside_perimeter(t: TilePos, x_max: u32, perimeter: &HashSet<TilePos>) -> bool {
				let mut n: u32 = 0;
				for x in t.x..=x_max+1 {
					if perimeter.contains(&TilePos { x, y: t.y }) && perimeter.contains(&TilePos { x, y: t.y+1 }) && perimeter.contains(&TilePos { x, y: t.y-1 }) {
						n += 1;
					}
				}
				n % 2 == 1
			}
			let x_min = red_tiles[i].x.min(red_tiles[j].x);
			let x_max = red_tiles[i].x.max(red_tiles[j].x);
			let y_min = red_tiles[i].y.min(red_tiles[j].y);
			let y_max = red_tiles[i].y.max(red_tiles[j].y);
			let mut is_ok = true;
			/*if is_ok*/ {
				let x = x_min;
				for y in y_min..=y_max {
					let tile_pos = TilePos { x, y };
					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
						is_ok = false;
						break
					}
				}
			}
			if is_ok {
				let x = x_max;
				for y in y_min..=y_max {
					let tile_pos = TilePos { x, y };
					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
						is_ok = false;
						break
					}
				}
			}
			if is_ok {
				let y = y_min;
				for x in x_min..=x_max {
					let tile_pos = TilePos { x, y };
					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
						is_ok = false;
						break
					}
				}
			}
			if is_ok {
				let y = y_max;
				for x in x_min..=x_max {
					let tile_pos = TilePos { x, y };
					if !is_on_perimeter(&tile_pos, &red_or_green_tiles) && !is_inside_perimeter(tile_pos, effective_x_max[y as usize], &red_or_green_tiles) {
						is_ok = false;
						break
					}
				}
			}
			if is_ok {
				dbg!(area);
			}
			is_ok.then_some(area)
		})
		.max()
	;
	max_area.unwrap()

}



fn parse_input(input: &str) -> Vec<TilePos> {
	input.lines().map(|line| {
		let (x, y) = line.split_once(',').unwrap();
		let x = x.parse().unwrap();
		let y = y.parse().unwrap();
		TilePos { x, y }
	}).collect()
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct TilePos {
	x: u32,
	y: u32,
}
impl TilePos {
	fn area_with(&self, other: &Self) -> u64 {
		let w = (self.x.abs_diff(other.x) + 1) as u64;
		let h = (self.y.abs_diff(other.y) + 1) as u64;
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

