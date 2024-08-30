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
			xp -= self.next_level_xp - self.xp_within_level;
			self.xp_within_level = 0;
			self.next_level_xp = (self.algorithm)(self.current_level, self.next_level_xp)?;
		}
		self.xp_within_level += xp;
		Ok(self)
	}
}
