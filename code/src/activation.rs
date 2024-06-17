use std::f64::consts::E;

pub fn sigmoid_function(x: f64) -> f64 {
	1.0 / (1.0 + E.powf(-x))
}

pub fn tanh_function(x: f64) -> f64 {
	x.tanh()
}

pub fn relu_function(x: f64) -> f64 {
	x.max(0.0)
}

pub fn leakyrelu_function(x: f64) -> f64 {
	match x > 0.0 {
		true => x,
		false => x * 0.01,
	}
}

pub fn elu_function(x: f64) -> f64 {
	match x > 0.0 {
		true => x,
		false => x.exp() - 1.0,
	}
}

pub fn softplus_function(x: f64) -> f64 {
	(1.0 + x.exp()).ln()
}

pub fn silu_function(x: f64) -> f64 {
	x / (1.0 + (-x).exp())
}

pub fn linear_function(x: f64) -> f64 {
	x
}

pub fn sigmoid_derivative(x: f64) -> f64 {
	let a = sigmoid_function(x);
	a * (1.0 - a)
}

pub fn tanh_derivative(x: f64) -> f64 {
	1.0 - x.tanh().powf(2.0)
}

pub fn relu_derivative(x: f64) -> f64 {
	match x > 0.0 {
		true => 1.0,
		false => 0.0,
	}
}

pub fn leakyrelu_derivative(x: f64) -> f64 {
	match x > 0.0 {
		true => 1.0,
		false => 0.01,
	}
}

pub fn elu_derivative(x: f64) -> f64 {
	match x > 0.0 {
		true => 1.0,
		false => x.exp(),
	}
}

pub fn softplus_derivative(x: f64) -> f64 {
	1.0 / (1.0 + (-x).exp())
}

pub fn silu_derivative(x: f64) -> f64 {
	let a = sigmoid_function(x);
	a * (1.0 + x * (1.0 - a))
}

pub fn linear_derivative(_x: f64) -> f64 {
	1.0
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
