type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub struct LevelSystem {
	pub starting_level: u128,
	pub current_level: u128,
	pub xp_per_level: Vec<u128>,
	pub xp_in_level: u128,
	pub xp_to_level: u128,
	algorithm: Box<dyn Fn(u128, u128) -> Result<u128>>,
}

impl LevelSystem {
	pub fn new(
		starting_level: u128, first_level_xp: u128,
		algorithm: Box<dyn Fn(u128, u128) -> Result<u128>>,
	) -> Self {
		LevelSystem {
			starting_level,
			current_level: starting_level,
			xp_per_level: vec![first_level_xp],
			xp_in_level: 0,
			xp_to_level: first_level_xp,
			algorithm,
		}
	}

	pub fn add_xp(mut self, xp: &mut u128) -> Result<Self> {
		while xp >= &mut self.xp_to_level {
			let next_level =
				(self.algorithm)(self.current_level + 1, *self.xp_per_level.last().unwrap())?;
		}
		Ok(self)
	}
}
