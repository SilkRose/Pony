use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use wiwi::clock_timer::{now, ClockTimer, TimeDelta, Timelike, SECS_PER_DAY};

struct Chapter {
	id: u32,
	release_time: u32,
	description: Option<String>,
	short_description: Option<String>,
}

const CHAPTERS: &[Chapter] = &[Chapter {
	id: 10,
	release_time: 10,
	description: None,
	short_description: None,
}];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let story_id = 552650;
	let api_domain = "https://www.fimfiction.net/api/v2/stories";

	let token = &env::args().collect::<Vec<_>>()[1];
	let client = Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

	let api_url = format!("{api_domain}/{story_id}");

	let mut timer = ClockTimer::with_naive_datetime(now());
	timer.set_run_duration(TimeDelta::try_seconds(SECS_PER_DAY as _).unwrap());
	timer.set_run_interval(TimeDelta::try_minutes(1).unwrap());

	while let Some(time) = timer.tick().await {
		let hour = time.hour();
		let min = time.minute();
		println!("{hour}, {min}");
		let remaining_mins = 60 - time.minute();
		let remaining_hours = if remaining_mins == 60 {
			24 - time.hour()
		} else {
			23 - time.hour()
		};
		let remaining_mins = if remaining_mins == 60 {
			0
		} else {
			remaining_mins
		};
		println!("{remaining_hours}, {remaining_mins}");
		let title = format!(
			"This Story will Explode in {remaining_hours} Hours and {remaining_mins} Minutes"
		);
		let story_json = story_json(story_id, title, None, None).to_string();
		//let body = chapter_json(story_id).to_string();
		println!("{story_json}");
		let _ = send_api_request(&client, &headers, &api_url, story_json).await;
	}

	Ok(())
}

async fn send_api_request(
	client: &Client, headers: &HeaderMap, url: &String, body: String,
) -> Result<(), Box<dyn std::error::Error>> {
	let api_response = client
		.patch(url)
		.headers(headers.clone())
		.body(body)
		.send()
		.await?;
	let api = api_response.text().await;
	println!("{:#?}", api);
	Ok(())
}

fn chapter_json(id: u32) -> Value {
	json!({
		 "data": {
			  "id": id,
			  "attributes": {
					"published": true
			  }
		 }
	})
}

fn story_json(
	id: u32, title: String, description: Option<String>, short_description: Option<String>,
) -> Value {
	match (description, short_description) {
		(None, None) => json!({
			  "data": {
				   "id": id,
				   "attributes": {
						"title": title
				   }
			  }
		}),
		(None, Some(short_description)) => json!({
			   "data": {
					"id": id,
					"attributes": {
						"title": title,
					   "short_description": short_description
					}
			   }
		}),
		(Some(description), None) => json!({
				"data": {
					"id": id,
					"attributes": {
						"title": title,
						"description": description
					}
				}
		}),
		(Some(description), Some(short_description)) => json!({
				"data": {
					"id": id,
					"attributes": {
						"title": title,
						"short_description": short_description,
						"description": description
					}
				}
		}),
	}
}
