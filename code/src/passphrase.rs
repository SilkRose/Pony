use rand::RngExt;

pub fn generate_passphrase() -> String {
	let characters = [
		"TwilightSparkle",
		"Rarity",
		"PinkiePie",
		"Fluttershy",
		"RainbowDash",
		"Applejack",
		"MayorMare",
		"DoctorWhooves",
		"Derpy",
		"Coloratura",
		"DaringDo",
		"PhotoFinish",
		"FancyPants",
		"SapphireShores",
		"Spitfire",
		"Soarin",
		"SunnyStarscout",
		"IzzyMoonbow",
		"QueenChrysalis",
		"SilkRose",
	];
	let actions = [
		"Boops",
		"Kisses",
		"Likes",
		"Loves",
		"WantsToDate",
		"Hugs",
		"Holds",
		"TalksTo",
		"SingsTo",
		"AsksOut",
		"GivesABitTo",
		"HasCoffeeWith",
		"HasAVerySpecificQuestionFor",
		"IsNotTheSamePonyAs",
		"HasACrushOn",
		"IsMarrying",
		"TripsInFrontOf",
		"LooksAt",
		"HoldsHoovesWith",
	];

	let mut rng = rand::rng();

	let char1 = characters[rng.random_range(0..characters.len())];
	let char2 = characters[rng.random_range(0..characters.len())];

	if char1 == char2 {
		format!("{char1}IsTheSamePonyAs{char2}")
	} else {
		let action = actions[rng.random_range(0..actions.len())];
		format!("{char1}{action}{char2}")
	}
}
