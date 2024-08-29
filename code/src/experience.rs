type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub struct LevelSystem {
	pub starting_level: i128,
	pub current_level: i128,
	pub xp_per_level: Vec<i128>,
	pub xp_in_level: i128,
	pub xp_to_level: i128,
	pub algorithm: Box<dyn Fn(i128, i128) -> Result<i128>>,
}

impl LevelSystem {
	pub fn new(
		starting_level: i128, first_level_xp: i128,
		algorithm: Box<dyn Fn(i128, i128) -> Result<i128>>,
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

	pub fn add_xp(mut self, xp: &mut i128) -> Result<Self> {
		while xp >= &mut self.xp_to_level {
			let next_level =
				(self.algorithm)(self.current_level + 1, *self.xp_per_level.last().unwrap())?;
		}
		Ok(self)
	}
}
