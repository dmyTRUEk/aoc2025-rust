//! Advent of Code - Day 12 Part 1 solution.
//! $ cargo run --bin dayXpY

#![allow(
	clippy::identity_op,
)]

#![deny(
	unused_assignments,
	unused_results,
)]

use std::{collections::HashSet, fmt::Display, fs::read_to_string as read_file_to_string};

// use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

// use aoc2025_rust::utils::{};



fn main() {
	let solution = solve_file("./input/day12.input");
	println!("{solution}");
}



fn solve_file(filepath: &str) -> u64 {
	let text = read_file_to_string(filepath).unwrap();
	solve_text(&text)
}



fn solve_text(text: &str) -> u64 {
	let (shapes, regions) = parse_input(text);

	// println!("shapes:");
	// for shape in shapes {
	// 	println!("{shape}\n");
	// }

	fn try_fill(
		region: Region,
		area: Vec<bool>,
		shapes: &[Shape; 6],
		depth: u32,
		checked_states: &mut HashSet<(Region, Vec<bool>)>,
	) -> bool {
		fn yx_to_index(y: u8, x: u8, w: u8) -> usize { ( (x as u16) + (w as u16)*(y as u16) ) as usize }
		if checked_states.contains(&(region, area.clone())) {
			return false
		}
		// println!("depth: {depth}, region.nums = {:?}", region.nums);
		if region.nums == [0; 6] { return true }
		for (shape_i, num) in region.nums.iter().enumerate() {
			if *num > 0 {
				// println!("shape:\n{shape}");
				for y in 0..region.h-2 {
					// let y = y as usize;
					for x in 0..region.w-2 {
						// println!("y={y}, x={x}");
						// let x = x as usize;
						'mark:
						for shape in shapes[shape_i].ats.iter() {
							let mut area = area.clone();
							//
							for dy in 0..3_u8 {
								for dx in 0..3_u8 {
									if shape.at[dy as usize][dx as usize] {
										let i = yx_to_index(y+dy, x+dx, region.w);
										if !area[i] {
											area[i] = true;
										} else {
											continue 'mark
										}
									}
								}
							}
							//
							let mut region = region;
							region.nums[shape_i] -= 1;
							let r = try_fill(region, area.clone(), shapes, depth+1, checked_states);
							if r { return true } else {
								// let _r = checked_states.insert((region, area));
								// assert!(!r);
							}
						}
					}
				}
			}
		}
		let _r = checked_states.insert((region, area));
		// assert!(!r);
		false
	}

	regions
		// .into_iter()
		.into_par_iter()
		.enumerate()
		.map(|(i, region)| {
			let area = vec![false; region.w as usize * region.h as usize];
			let r = try_fill(region, area, &shapes, 0, &mut HashSet::new());
			println!("regions[{i}]: {r}");
			r
		})
		.filter(|is_ok| *is_ok)
		.count()
		.try_into().unwrap()
}



fn parse_input(input: &str) -> ([Shape; 6], Vec<Region>) {
	let (shapes, regions) = input.rsplit_once("\n\n").unwrap();
	let shapes: [Shape; 6] = shapes.split("\n\n").map(|shape_str| {
		let (_index, shape_str) = shape_str.split_once('\n').unwrap();
		Shape::from_str(shape_str)
	}).collect::<Vec<Shape>>().try_into().unwrap();
	let regions: Vec<Region> = regions.lines().map(|region_str| {
		Region::from_str(region_str)
	}).collect();
	(shapes, regions)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ShapeExact {
	at: [[bool; 3]; 3]
}
impl ShapeExact {
	fn from_str(s: &str) -> Self {
		let at: [[bool; 3]; 3] = s.lines()
			.map(|line| {
				line.bytes().map(|byte| {
					match byte {
						b'#' => true,
						b'.' => false,
						_ => unreachable!()
					}
				}).collect::<Vec<bool>>().try_into().unwrap()
			}).collect::<Vec<[bool; 3]>>().try_into().unwrap();
		Self { at }
	}
}
impl Display for ShapeExact {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fn char(b: bool) -> char { if b { '#' } else { '.' } }
		let at = self.at;
		writeln!(f, "{}{}{}", char(at[0][0]), char(at[0][1]), char(at[0][2])).unwrap();
		writeln!(f, "{}{}{}", char(at[1][0]), char(at[1][1]), char(at[1][2])).unwrap();
		write!  (f, "{}{}{}", char(at[2][0]), char(at[0][1]), char(at[2][2])).unwrap();
		Ok(())
	}
}

#[derive(Debug, Clone)]
struct Shape {
	ats: Vec<ShapeExact>
}
impl Shape {
	fn from_str(s: &str) -> Self {
		Self::new(ShapeExact::from_str(s))
	}
	fn new(mut at: ShapeExact) -> Self {
		let mut ats = vec![at];
		for _rotation_i in 0..3 {
			at = rotate_once(at);
			ats.push(at);
		}
		at = flip_w(at);
		for _rotation_i in 0..3 {
			at = rotate_once(at);
			ats.push(at);
		}
		ats.sort();
		ats.dedup();
		Self { ats }
	}
}
impl Display for Shape {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fn char(b: bool) -> char { if b { '#' } else { '.' } }
		for (i, shape) in self.ats.iter().enumerate() {
			write!(f, "{s}{}{}{}",
				char(shape.at[0][0]),
				char(shape.at[0][1]),
				char(shape.at[0][2]),
				s = if i > 0 { " " } else { "" },
			).unwrap();
		}
		writeln!(f).unwrap();
		for (i, shape) in self.ats.iter().enumerate() {
			write!(f, "{s}{}{}{}",
				char(shape.at[1][0]),
				char(shape.at[1][1]),
				char(shape.at[1][2]),
				s = if i > 0 { " " } else { "" },
			).unwrap();
		}
		writeln!(f).unwrap();
		for (i, shape) in self.ats.iter().enumerate() {
			write!(f, "{s}{}{}{}",
				char(shape.at[2][0]),
				char(shape.at[2][1]),
				char(shape.at[2][2]),
				s = if i > 0 { " " } else { "" },
			).unwrap();
		}
		Ok(())
	}
}

fn rotate_once(ShapeExact { at: at_old }: ShapeExact) -> ShapeExact {
	let mut at_new = at_old;
	at_new[0][0] = at_old[0][2];
	at_new[0][2] = at_old[2][2];
	at_new[2][2] = at_old[2][0];
	at_new[2][0] = at_old[0][0];
	//
	at_new[0][1] = at_old[1][2];
	at_new[1][2] = at_old[2][1];
	at_new[2][1] = at_old[1][0];
	at_new[1][0] = at_old[0][1];
	ShapeExact { at: at_new }
}

fn flip_w(ShapeExact { at: at_old }: ShapeExact) -> ShapeExact {
	let mut at_new = at_old;
	at_new[0][0] = at_old[0][2];
	at_new[0][2] = at_old[0][0];
	//
	at_new[1][0] = at_old[1][2];
	at_new[1][2] = at_old[1][0];
	//
	at_new[2][0] = at_old[2][2];
	at_new[2][2] = at_old[2][0];
	ShapeExact { at: at_new }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Region {
	w: u8,
	h: u8,
	nums: [u8; 6],
}
impl Region {
	fn from_str(s: &str) -> Self {
		let (wh_str, nums_str) = s.split_once(": ").unwrap();
		let (w_str, h_str) = wh_str.split_once('x').unwrap();
		let w: u8 = w_str.parse().unwrap();
		let h: u8 = h_str.parse().unwrap();
		let nums: Vec<u8> = nums_str.split(' ')
			.map(|n_str| n_str.parse::<u8>().unwrap())
			.collect();
		let nums = nums.try_into().unwrap();
		Region { w, h, nums }
	}
}





// $ cargo test --bin dayXpY
#[test]
fn example_1() {
	assert_eq!(
		2,
		solve_file("./input/day12.example")
	)
}

