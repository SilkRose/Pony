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
	/// Amount to scale x by before calculation.
	pub scale: f64,
	/// Minimum value for any noise result.
	pub minimum: f64,
	/// Maximum value for any noise result.
	pub maximum: f64,
	/// Spacing between lattices.
	pub frequency: f64,
	/// How much each octave contributes.
	pub amplitude: f64,
	/// Frequency multiplier for each octave.
	pub lacunarity: f64,
	/// How much to adjust amplitude by.
	pub persistence: f64,
	/// Number of octaves to combine.
	pub octaves: u8,
}

impl PerlinNoise1D {
	pub fn get_point(&self, x: f64) -> Result<f64> {
		let mut x = x * self.scale;
		let mut noise = 0.0;
		let mut max_noise = 0.0;
		let maximum = self.maximum - self.minimum;
		let mut frequency = self.frequency;
		let mut amplitude = self.amplitude;
		for _ in 0..self.octaves {
			let lower_lattice = x.floor();
			let upper_lattice = lower_lattice + 1.0;
			let mut rng =
				StdRng::seed_from_u64(self.seed.wrapping_add_signed(lower_lattice as i64));
			let lower_noise = rng.gen_range(self.minimum..=self.maximum);
			rng = StdRng::seed_from_u64(self.seed.wrapping_add_signed(upper_lattice as i64));
			let upper_noise = rng.gen_range(self.minimum..=self.maximum);
			let interpolation_factor = fade(x - lower_lattice);
			let n = lerp(interpolation_factor, lower_noise, upper_noise);
			noise += n * amplitude;
			max_noise += amplitude * maximum;
			frequency *= self.lacunarity;
			amplitude *= self.persistence;
			x *= frequency;
		}
		noise = maximum * (noise / max_noise);
		Ok(noise)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use image::{ImageBuffer, Rgba, RgbaImage};
	use std::collections::HashSet;

	#[test]
	fn one_dimension() -> Result<()> {
		for octave in 1..=8 {
			let noise = PerlinNoise1D {
				seed: 7669,
				scale: 0.01,
				minimum: 0.0,
				maximum: 100.0,
				frequency: 1.2,
				amplitude: 1.0,
				lacunarity: 2.0,
				persistence: 0.5,
				octaves: octave,
			};
			let mut values = vec![];
			for x in 0..1000 {
				values.push(noise.get_point(x as f64)?.round() as u32);
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

	#[test]
	fn has_test() {
		let seed = 7669_u64;
		let mut values = HashSet::with_capacity(300_000_000);
		for x in 0..300_i64 {
			for y in 0..1_i64 {
				let seed = seed.wrapping_add_signed(x << 32 | y);
				values.insert(seed);
			}
		}
		println!("{}", values.len());
		println!("{}", i64::MAX >> 32);
	}
}
