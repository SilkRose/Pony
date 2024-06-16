use std::fmt;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Matrix {
	pub rows: usize,
	pub cols: usize,
	pub data: Vec<f64>,
}

impl Matrix {
	pub fn zeros(rows: usize, cols: usize) -> Matrix {
		Matrix {
			rows,
			cols,
			data: vec![0.0; rows * cols],
		}
	}

	pub fn random(rows: usize, cols: usize) -> Matrix {
		let mut data: Vec<f64> = Vec::with_capacity(rows * cols);
		for _ in 0..rows * cols {
			data.push(thread_rng().gen_range(0.0..=1.0));
		}
		Matrix { rows, cols, data }
	}

	pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {
		if data.len() != rows * cols {
			panic!("Invalid matrix size!")
		}
		Matrix { rows, cols, data }
	}

	pub fn element_wise_multiply(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Mismatched Matricies!")
		}
		let mut result = Matrix::zeros(self.rows, self.cols);
		for i in 0..self.data.len() {
			result.data[i] = self.data[i] * other.data[i];
		}
		result
	}

	pub fn add(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Mismatched Matricies!")
		}
		let mut result = Matrix::zeros(self.rows, self.cols);
		for i in 0..self.data.len() {
			result.data[i] = self.data[i] + other.data[i];
		}
		result
	}

	pub fn subtract(&self, other: &Matrix) -> Matrix {
		if self.rows != other.rows || self.cols != other.cols {
			panic!("Mismatched Matricies!")
		}
		let mut result = Matrix::zeros(self.rows, self.cols);
		for i in 0..self.data.len() {
			result.data[i] = self.data[i] - other.data[i];
		}
		result
	}

	pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
		if self.cols != other.rows {
			panic!("Mismatched Matricies!")
		}
		let mut result = Matrix::zeros(self.rows, other.cols);
		for i in 0..self.rows {
			for j in 0..other.cols {
				let mut sum = 0.0;
				for k in 0..self.cols {
					sum += self.data[i * self.cols + k] * other.data[k * other.cols + j]
				}
				result.data[i * other.cols + j] = sum;
			}
		}
		result
	}

	pub fn transpose(&self) -> Matrix {
		let mut result = Matrix::zeros(self.cols, self.rows);
		for i in 0..self.rows {
			for j in 0..self.cols {
				result.data[j * self.rows + i] = self.data[i * self.cols + j];
			}
		}
		result
	}

	pub fn map(&self, fun: fn(f64) -> f64) -> Matrix {
		let mut result = Matrix::zeros(self.rows, self.cols);
		for i in 0..self.data.len() {
			result.data[i] = fun(self.data[i])
		}
		result
	}
}

impl From<Vec<f64>> for Matrix {
	fn from(vec: Vec<f64>) -> Self {
		Matrix {
			rows: vec.len(),
			cols: 1,
			data: vec,
		}
	}
}

impl PartialEq for Matrix {
	fn eq(&self, other: &Self) -> bool {
		self.rows == other.rows && self.cols == other.cols && self.data == other.data
	}
}

impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for row in 0..self.rows {
			for col in 0..self.cols {
				write!(f, "{}", self.data[row * self.cols + col])?;
				if col < self.cols - 1 {
					write!(f, "\t")?
				}
			}
			if row < self.rows - 1 {
				writeln!(f)?;
			}
		}
		Ok(())
	}
}
