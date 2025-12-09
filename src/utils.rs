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

