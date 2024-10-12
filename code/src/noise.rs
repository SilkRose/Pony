use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

fn fade(t: f64) -> f64 {
	t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f64, a: f64, b: f64) -> f64 {
	a + t * (b - a)
}

pub struct PerlinNoise1D {
	/// Seed value to generate noise from.
	pub seed: u64,
	/// Minimum value for any noise result.
	pub minimum: f64,
	/// Maximum value for any noise result.
	pub maximum: f64,
	/// Spacing between lattices.
	pub frequency: i64,
	/// How much each octave contributes.
	pub amplitude: f64,
	/// Frequency multiplier for each octave.
	pub lacunarity: f64,
	/// How much to adjust amplitude by.
	pub persistence: f64,
	/// Number of octaves to combine.
	pub octaves: u32,
}

impl PerlinNoise1D {
	pub fn get_point(&self, x: i64) -> Result<f64> {
		let mut noise = 0.0;
		let mut max_noise = 0.0;
		let maximum = self.maximum - self.minimum;
		let mut frequency = self.frequency;
		let mut amplitude = self.amplitude;
		for _ in 0..self.octaves {
			let lower_lattice = x - (x % frequency);
			let upper_lattice = lower_lattice + frequency;
			let mut rng = StdRng::seed_from_u64(self.seed.wrapping_add_signed(lower_lattice));
			let lower_noise = rng.gen_range(self.minimum..=self.maximum);
			rng = StdRng::seed_from_u64(self.seed.wrapping_add_signed(upper_lattice));
			let upper_noise = rng.gen_range(self.minimum..=self.maximum);
			let interpolation_factor = (x - lower_lattice) as f64 / frequency as f64;
			let interpolation_factor = fade(interpolation_factor);
			let n = (1.0 - interpolation_factor) * lower_noise + interpolation_factor * upper_noise;
			noise += n * amplitude;
			max_noise += amplitude * maximum;
			frequency *= self.lacunarity as i64;
			amplitude *= self.persistence;
		}
		noise = maximum * (noise / max_noise);
		Ok(noise)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use image::{ImageBuffer, Rgba, RgbaImage};

	#[test]
	fn one_dimension() -> Result<()> {
		for octave in 1..=8 {
			let noise = PerlinNoise1D {
				seed: 7669,
				minimum: 0.0,
				maximum: 100.0,
				frequency: 64,
				amplitude: 1.0,
				lacunarity: 4.0,
				persistence: 0.8,
				octaves: octave,
			};
			let mut values = vec![];
			for x in 0..1000 {
				values.push(noise.get_point(x)?.round() as u32);
			}
			let image: RgbaImage = ImageBuffer::from_fn(1000, 100, |x, y| {
				if y > values[x as usize] {
					Rgba([0, 0, 0, 255])
				} else {
					Rgba([255, 255, 255, 255])
				}
			});
			image.save(format!("{octave}.png"))?;
		}
		Ok(())
	}
}
