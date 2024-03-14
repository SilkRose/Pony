use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::env;
use wiwi::clock_timer::{now, ClockTimer, TimeDelta, Timelike, SECS_PER_DAY};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let id = 539654;
	let api_domain = "https://www.fimfiction.net/api/v2/stories";

	let token = &env::args().collect::<Vec<_>>()[1];
	let client = Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

	let api_url = format!("{api_domain}/{id}");

	let mut timer = ClockTimer::with_naive_datetime(now());
	timer.set_run_duration(TimeDelta::try_seconds(SECS_PER_DAY as _).unwrap());
	timer.set_run_interval(TimeDelta::try_minutes(1).unwrap());

	while let Some(time) = timer.tick().await {
		let hour = time.hour();
		let min = time.minute();
		println!("{hour}, {min}");
	}

	let api_response = client.get(api_url).headers(headers.clone()).send().await?;

	let api = api_response.text().await;
	println!("{:#?}", api);

	Ok(())
}
