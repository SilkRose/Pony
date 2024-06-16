use std::f64::consts::E;

pub trait ActivationFunctions {
	fn sigmoid_function(self) -> f64;
	fn tanh_function(self) -> f64;
	fn relu_function(self) -> f64;
	fn leakyrelu_function(self) -> f64;
	fn elu_function(self) -> f64;
	fn softplus_function(self) -> f64;
	fn silu_function(self) -> f64;
	fn linear_function(self) -> f64;
}

impl ActivationFunctions for f64 {
	fn sigmoid_function(self) -> f64 {
		1.0 / (1.0 + E.powf(-self))
	}

	fn tanh_function(self) -> f64 {
		self.tanh()
	}

	fn relu_function(self) -> f64 {
		self.max(0.0)
	}

	fn leakyrelu_function(self) -> f64 {
		match self > 0.0 {
			true => self,
			false => self * 0.01,
		}
	}

	fn elu_function(self) -> f64 {
		match self > 0.0 {
			true => self,
			false => self.exp() - 1.0,
		}
	}

	fn softplus_function(self) -> f64 {
		(1.0 + self.exp()).ln()
	}

	fn silu_function(self) -> f64 {
		self / (1.0 + (-self).exp())
	}

	fn linear_function(self) -> f64 {
		self
	}
}

pub trait ActivationDerivitives {
	fn sigmoid_derivative(self) -> f64;
	fn tanh_derivative(self) -> f64;
	fn relu_derivative(self) -> f64;
	fn leakyrelu_derivative(self) -> f64;
	fn elu_derivative(self) -> f64;
	fn softplus_derivative(self) -> f64;
	fn silu_derivative(self) -> f64;
	fn linear_derivative(self) -> f64;
}

impl ActivationDerivitives for f64 {
	fn sigmoid_derivative(self) -> f64 {
		let a = self.sigmoid_function();
		a * (1.0 - a)
	}

	fn tanh_derivative(self) -> f64 {
		1.0 - self.tanh().powf(2.0)
	}

	fn relu_derivative(self) -> f64 {
		match self > 0.0 {
			true => 1.0,
			false => 0.0,
		}
	}

	fn leakyrelu_derivative(self) -> f64 {
		match self > 0.0 {
			true => 1.0,
			false => 0.01,
		}
	}

	fn elu_derivative(self) -> f64 {
		match self > 0.0 {
			true => 1.0,
			false => self.exp(),
		}
	}

	fn softplus_derivative(self) -> f64 {
		1.0 / (1.0 + (-self).exp())
	}

	fn silu_derivative(self) -> f64 {
		let a = self.sigmoid_function();
		a * (1.0 + self * (1.0 - a))
	}

	fn linear_derivative(self) -> f64 {
		1.0
	}
}

pub fn softmax_function(values: &[f64]) -> Vec<f64> {
	let sum = values.iter().map(|i| i.exp()).sum::<f64>();
	values.iter().map(|i| i.exp() / sum).collect()
}

pub fn softmax_derivative(values: &[f64]) -> Vec<Vec<f64>> {
	let softmax_values = softmax_function(values);
	let n = softmax_values.len();
	let mut result = vec![vec![0.0; n]; n];
	for i in 0..n {
		for j in 0..n {
			if i == j {
				result[i][j] = softmax_values[i] * (1.0 - softmax_values[i]);
			} else {
				result[i][j] = -softmax_values[i] * softmax_values[j];
			}
		}
	}
	result
}
