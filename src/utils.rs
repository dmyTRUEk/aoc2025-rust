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

