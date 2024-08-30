type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub struct LevelSystem<T> {
	pub current_level: T,
	pub next_level_xp: T,
	pub xp_within_level: T,
	algorithm: Box<dyn Fn(T, T) -> Result<T>>,
}

macro_rules! experience {
	($($T:ty),+ $(,)?) => {
		$(impl LevelSystem<$T> {
			pub fn new(
				starting_level: $T, first_level_xp: $T,
				algorithm: Box<dyn Fn($T, $T) -> Result<$T>>,
			) -> Self {
				LevelSystem {
					current_level: starting_level,
					next_level_xp: first_level_xp,
					xp_within_level: 0,
					algorithm,
				}
			}

			pub fn add_xp(&mut self, xp: $T) -> Result<()> {
				let mut xp = xp;
				while xp >= self.next_level_xp {
					self.current_level += 1;
					xp -= self.next_level_xp - self.xp_within_level;
					self.xp_within_level = 0;
					self.next_level_xp = (self.algorithm)(self.current_level, self.next_level_xp)?;
				}
				self.xp_within_level += xp;
				Ok(())
			}
		})+
	};
}

experience!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn xp() -> Result<()> {
		let mut xp = LevelSystem::<u128>::new(0, 100, Box::new(|level, _| Ok((level + 1) * 100)));
		xp.add_xp(1000)?;
		assert_eq!(4, xp.current_level);
		Ok(())
	}

	#[test]
	fn progress() -> Result<()> {
		let mut xp = LevelSystem::<u128>::new(0, 100, Box::new(|level, _| Ok((level + 1) * 100)));
		xp.add_xp(200)?;
		assert_eq!(100, xp.xp_within_level);
		Ok(())
	}

	#[test]
	fn max() -> Result<()> {
		let mut xp = LevelSystem::<u128>::new(0, u128::MAX, Box::new(|_, _| Ok(u128::MAX)));
		xp.add_xp(u128::MAX)?;
		assert_eq!(1, xp.current_level);
		Ok(())
	}
}
