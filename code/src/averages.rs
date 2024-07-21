use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SimpleMovingAverage {
	pub data: VecDeque<f64>,
	pub window_size: usize,
}

impl SimpleMovingAverage {
	pub fn new(window_size: usize) -> SimpleMovingAverage {
		SimpleMovingAverage {
			data: VecDeque::with_capacity(window_size),
			window_size,
		}
	}

	pub fn insert(&mut self, value: f64) {
		while self.data.len() >= self.window_size {
			self.data.pop_front();
		}
		self.data.push_back(value)
	}

	pub fn average(&self) -> Option<f64> {
		if !self.data.is_empty() {
			Some(self.data.iter().sum::<f64>() / self.data.len() as f64)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn simple_moving_average() {
		let mut sma = SimpleMovingAverage::new(2);
		sma.insert(2.0);
		sma.insert(2.0);
		assert_eq!(Some(2.0), sma.average())
	}
}
