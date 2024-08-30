type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub struct LevelSystem {
	pub current_level: u128,
	pub next_level_xp: u128,
	pub xp_within_level: u128,
	algorithm: Box<dyn Fn(u128, u128) -> Result<u128>>,
}

impl LevelSystem {
	pub fn new(
		starting_level: u128, first_level_xp: u128,
		algorithm: Box<dyn Fn(u128, u128) -> Result<u128>>,
	) -> Self {
		LevelSystem {
			current_level: starting_level,
			next_level_xp: first_level_xp,
			xp_within_level: 0,
			algorithm,
		}
	}

	pub fn add_xp(mut self, xp: u128) -> Result<Self> {
		let mut xp = xp;
		while xp >= self.next_level_xp {
			self.current_level += 1;
			xp -= self.next_level_xp - self.xp_within_level;
			self.xp_within_level = 0;
			self.next_level_xp = (self.algorithm)(self.current_level, self.next_level_xp)?;
		}
		self.xp_within_level += xp;
		Ok(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn xp() -> Result<()> {
		let xp =
			LevelSystem::new(0, 100, Box::new(|level, _| Ok((level + 1) * 100))).add_xp(1000)?;
		assert_eq!(4, xp.current_level);
		Ok(())
	}

	#[test]
	fn progress() -> Result<()> {
		let xp =
			LevelSystem::new(0, 100, Box::new(|level, _| Ok((level + 1) * 100))).add_xp(200)?;
		assert_eq!(100, xp.xp_within_level);
		Ok(())
	}

	#[test]
	fn max() -> Result<()> {
		let xp =
			LevelSystem::new(0, u128::MAX, Box::new(|_, _| Ok(u128::MAX))).add_xp(u128::MAX)?;
		assert_eq!(1, xp.current_level);
		Ok(())
	}
}
