//! utils



pub trait PositionFirstMax<T> {
	fn position_first_max(&self) -> Option<u32>;
}
impl<T: PartialOrd + Copy> PositionFirstMax<T> for &[T] {
	fn position_first_max(&self) -> Option<u32> {
		let mut max: Option<(u32, T)> = None;
		for (i, v) in self.iter().enumerate() {
			match max {
				None => {
					max = Some((i as u32, *v));
				}
				Some((_i_max, v_max)) if *v > v_max => {
					max = Some((i as u32, *v));
				}
				_ => {}
			}
		}
		max.map(|(i, _v)| i)
	}
}



pub trait VecFromFn<I, T> {
	fn from_fn<F: FnMut(I) -> T>(len: I, f: F) -> Self;
}
impl<T> VecFromFn<u32, T> for Vec<T> {
	fn from_fn<F: FnMut(u32) -> T>(len: u32, f: F) -> Self {
		(0..len).map(f).collect()
	}
}
impl<T> VecFromFn<usize, T> for Vec<T> {
	fn from_fn<F: FnMut(usize) -> T>(len: usize, f: F) -> Self {
		(0..len).map(f).collect()
	}
}



pub trait Bits<const N: usize> {
	fn bits(self) -> [bool; N];
}
impl Bits<32> for u32 {
	fn bits(mut self) -> [bool; 32] {
		let mut res = [false; 32];
		let mut i = 0;
		while self > 0 {
			if self % 2 == 1 {
				res[i] = true;
			}
			self /= 2;
			i += 1;
		}
		res.reverse();
		res
	}
}

#[cfg(test)]
mod bits {
	use super::*;
	fn bits<const N: usize>(bits: [u8; N]) -> [bool; N] {
		bits.map(|bit| {
			match bit {
				0 => false,
				1 => true,
				_ => unreachable!()
			}
		})
	}
	mod u32 {
		use super::*;
		#[test] fn _0() { assert_eq!(bits([0; 32]), 0_u32.bits()) }
		#[test] fn _1() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,1]), 1_u32.bits()) }
		#[test] fn _2() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,1,0]), 2_u32.bits()) }
		#[test] fn _3() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,1,1]), 3_u32.bits()) }
		#[test] fn _4() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,1,0,0]), 4_u32.bits()) }
		#[test] fn _5() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,1,0,1]), 5_u32.bits()) }
		#[test] fn _6() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,1,1,0]), 6_u32.bits()) }
		#[test] fn _7() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,1,1,1]), 7_u32.bits()) }
		#[test] fn _8() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,1,0,0,0]), 8_u32.bits()) }
		#[test] fn _9() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,1,0,0,1]), 9_u32.bits()) }
		#[test] fn _42() { assert_eq!(bits([0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,0,0,0,0,0,0 , 0,0,1,0,1,0,1,0]), 42_u32.bits()) }
		#[test] fn _max() { assert_eq!(bits([1; 32]), u32::MAX.bits()) }
	}
}





/// Iterator over all weak compositions of `s` into `n` non-negative parts.
/// Yields all `Vec<usize>` of length `n` whose elements sum to `s`.
pub struct Compositions {
	n: usize,
	s: usize,
	l: usize, // l = s + n - 1 (length of stars+bars sequence)
	k: usize, // k = n - 1 (number of bars)
	comb: Vec<usize>,
	done: bool,
}

impl Compositions {
	pub fn new(n: usize, s: usize) -> Self {
		assert!(n >= 1, "n must be >= 1");
		if n == 1 {
			// special-case: only one composition: [s]
			return Self { n, s, l: s, k: 0, comb: Vec::new(), done: false };
		}
		let k = n - 1;
		let l = s + n - 1;
		let mut comb = Vec::with_capacity(k);
		// initial combination: [0, 1, 2, ..., k-1]
		for i in 0..k { comb.push(i); }
		Self { n, s, l, k, comb, done: false }
	}
}

impl Iterator for Compositions {
	type Item = Vec<usize>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.done {
			return None;
		}

		// n == 1 case
		if self.n == 1 {
			self.done = true;
			return Some(vec![self.s]);
		}

		// Convert current combination of bar positions into composition:
		// L = l, k = n-1, comb = b0..b_{k-1}
		let l = self.l;
		let k = self.k;
		let b = &self.comb;

		let mut comp = Vec::with_capacity(self.n);
		// first part: number of stars before first bar = b0
		comp.push(b[0]);
		// middle parts:
		for i in 1..k {
			comp.push(b[i] - b[i - 1] - 1);
		}
		// last part: number of stars after last bar
		comp.push((l - 1) - b[k - 1]);

		// Prepare next combination (lexicographic)
		// max value for comb[i] is L - k + i
		let mut idx = None;
		for i in (0..k).rev() {
			if b[i] < (l - k + i) {
				idx = Some(i);
				break;
			}
		}

		if let Some(i) = idx {
			// increment comb[i] and set following to consecutive values
			self.comb[i] += 1;
			for j in i+1..k {
				self.comb[j] = self.comb[j - 1] + 1;
			}
		} else {
			// last combination has been produced
			self.done = true;
		}

		Some(comp)
	}
}

#[cfg(test)]
mod compositions {
	use super::*;
	mod n_eq_1 {
		const N: usize = 1;
		use super::*;
		#[test]
		fn s_eq_0() {
			let s = 0;
			assert_eq!(
				vec![vec![0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_1() {
			let s = 1;
			assert_eq!(
				vec![vec![1]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_2() {
			let s = 2;
			assert_eq!(
				vec![vec![2]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_3() {
			let s = 3;
			assert_eq!(
				vec![vec![3]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_4() {
			let s = 4;
			assert_eq!(
				vec![vec![4]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
	}
	mod n_eq_2 {
		const N: usize = 2;
		use super::*;
		#[test]
		fn s_eq_0() {
			let s = 0;
			assert_eq!(
				vec![vec![0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_1() {
			let s = 1;
			assert_eq!(
				vec![vec![0,1], vec![1,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_2() {
			let s = 2;
			assert_eq!(
				vec![vec![0,2], vec![1,1], vec![2,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_3() {
			let s = 3;
			assert_eq!(
				vec![vec![0,3], vec![1,2], vec![2,1], vec![3,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
	}
	mod n_eq_3 {
		const N: usize = 3;
		use super::*;
		#[test]
		fn s_eq_0() {
			let s = 0;
			assert_eq!(
				vec![vec![0,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_1() {
			let s = 1;
			assert_eq!(
				vec![vec![0,0,1], vec![0,1,0], vec![1,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_2() {
			let s = 2;
			assert_eq!(
				vec![vec![0,0,2], vec![0,1,1], vec![0,2,0], vec![1,0,1], vec![1,1,0], vec![2,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_3() {
			let s = 3;
			assert_eq!(
				vec![vec![0,0,3], vec![0,1,2], vec![0,2,1], vec![0,3,0], vec![1,0,2], vec![1,1,1], vec![1,2,0], vec![2,0,1], vec![2,1,0], vec![3,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
	}
	mod n_eq_4 {
		const N: usize = 4;
		use super::*;
		#[test]
		fn s_eq_0() {
			let s = 0;
			assert_eq!(
				vec![vec![0,0,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_1() {
			let s = 1;
			assert_eq!(
				vec![vec![0,0,0,1], vec![0,0,1,0], vec![0,1,0,0], vec![1,0,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_2() {
			let s = 2;
			assert_eq!(
				vec![vec![0,0,0,2], vec![0,0,1,1], vec![0,0,2,0], vec![0,1,0,1], vec![0,1,1,0], vec![0,2,0,0], vec![1,0,0,1], vec![1,0,1,0], vec![1,1,0,0], vec![2,0,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
		#[test]
		fn s_eq_3() {
			let s = 3;
			assert_eq!(
				vec![vec![0,0,0,3], vec![0,0,1,2], vec![0,0,2,1], vec![0,0,3,0], vec![0,1,0,2], vec![0,1,1,1], vec![0,1,2,0], vec![0,2,0,1], vec![0,2,1,0], vec![0,3,0,0], vec![1,0,0,2], vec![1,0,1,1], vec![1,0,2,0], vec![1,1,0,1], vec![1,1,1,0], vec![1,2,0,0], vec![2,0,0,1], vec![2,0,1,0], vec![2,1,0,0], vec![3,0,0,0]],
				Compositions::new(N, s).collect::<Vec<_>>()
			)
		}
	}
}

