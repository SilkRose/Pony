use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SimpleMovingAverage<T> {
	pub data: VecDeque<T>,
	pub window_size: usize,
}

macro_rules! simple_moving_average {
	($($T:ty),+ $(,)?) => {
		$(impl SimpleMovingAverage<$T> {
			pub fn new(window_size: usize) -> SimpleMovingAverage<$T> {
				SimpleMovingAverage {
					data: VecDeque::with_capacity(window_size),
					window_size,
				}
			}

			pub fn insert(&mut self, value: $T) {
				while self.data.len() >= self.window_size {
					self.data.pop_front();
				}
				self.data.push_back(value)
			}

			pub fn average(&self) -> Option<$T> {
				if !self.data.is_empty() {
					Some(self.data.iter().sum::<$T>() / self.data.len() as $T)
				} else {
					None
				}
			}

			pub fn change_window_size(&mut self, window_size: usize) {
				while self.data.len() > window_size {
					self.data.pop_front();
				}
				self.window_size = window_size;
			}

			pub fn clear_data(&mut self)  {
				self.data = VecDeque::with_capacity(self.window_size);
			}
		})+
	};
}

simple_moving_average!(f32, f64);
simple_moving_average!(u8, u16, u32, u64, u128, usize);
simple_moving_average!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn simple_moving_average_average() {
		let mut sma = SimpleMovingAverage::<f64>::new(2);
		sma.insert(2.0);
		sma.insert(2.0);
		assert_eq!(Some(2.0), sma.average())
	}
	#[test]
	fn simple_moving_average_change_window() {
		let mut sma = SimpleMovingAverage::<f64>::new(2);
		sma.insert(2.0);
		sma.insert(2.0);
		sma.change_window_size(1);
		assert_eq!(Some(2.0), sma.average())
	}
	#[test]
	fn simple_moving_average_clear_data() {
		let mut sma = SimpleMovingAverage::<f64>::new(2);
		sma.insert(2.0);
		sma.insert(2.0);
		sma.clear_data();
		assert_eq!(SimpleMovingAverage::<f64>::new(2).data, sma.data);
		assert_eq!(
			SimpleMovingAverage::<f64>::new(2).window_size,
			sma.window_size
		);
	}
	#[test]
	fn simple_moving_average_inser_after_full() {
		let mut sma = SimpleMovingAverage::<f64>::new(2);
		sma.insert(1.0);
		sma.insert(2.0);
		sma.insert(2.0);
		assert_eq!(Some(2.0), sma.average())
	}
	#[test]
	fn simple_moving_average_empty() {
		let sma = SimpleMovingAverage::<f64>::new(2);
		assert_eq!(None, sma.average())
	}
}
