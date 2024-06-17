use crate::matrix::Matrix;

pub struct Network {
	layers: Vec<usize>,
	weights: Vec<Matrix>,
	biases: Vec<Matrix>,
	data: Vec<Matrix>,
	activation: fn(f64) -> f64,
	derivative: fn(f64) -> f64,
	_learning_rate: f64,
}

impl Network {
	pub fn new(
		layers: Vec<usize>, activation: fn(f64) -> f64, derivative: fn(f64) -> f64,
		_learning_rate: f64,
	) -> Network {
		let mut weights: Vec<Matrix> = Vec::with_capacity(layers.len() - 1);
		let mut biases: Vec<Matrix> = Vec::with_capacity(layers.len() - 1);

		for i in 0..layers.len() - 1 {
			weights.push(Matrix::random(layers[i + 1], layers[i]));
			biases.push(Matrix::random(layers[i + 1], 1));
		}

		Network {
			layers,
			weights,
			biases,
			data: vec![],
			activation,
			derivative,
			_learning_rate,
		}
	}

	pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
		if self.layers[0] != inputs.data.len() {
			panic!("Mismatched input size and data!")
		}
		let mut current = inputs;
		self.data = vec![current.clone()];
		for i in 0..self.layers.len() - 1 {
			current = self.weights[i]
				.dot_multiply(&current)
				.add(&self.biases[i])
				.map(self.activation);
			self.data.push(current.clone())
		}
		current
	}

	pub fn back_propogate(&mut self, input: Matrix, target: Matrix) {
		let mut errors = target.subtract(&input);
		let mut gradients = input.clone().map(self.derivative);
		for i in (0..self.layers.len() - 1).rev() {
			gradients = gradients.element_wise_multiply(&errors).map(|x| x * 0.5);
			self.weights[i] =
				self.weights[i].add(&gradients.dot_multiply(&self.data[i].transpose()));
			self.biases[i] = self.biases[i].add(&gradients);
			errors = self.weights[i].transpose().dot_multiply(&errors);
			gradients = self.data[i].map(self.derivative);
		}
	}

	pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32) {
		if inputs.len() != targets.len() {
			panic!("Inputs and targets must have the same length");
		}
		for epoch in 1..=epochs {
			if epochs >= 100 && epoch % (epochs / 100) == 0 {
				println!("Epoch {} of {}", epoch, epochs);
			}
			for (input, target) in inputs.iter().zip(targets.iter()) {
				let input_matrix = Matrix::from(input.clone());
				let target_matrix = Matrix::from(target.clone());

				let outputs = self.feed_forward(input_matrix.clone());
				self.back_propogate(outputs, target_matrix);
			}
		}
	}
}
