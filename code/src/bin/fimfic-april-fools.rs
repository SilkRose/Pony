use async_recursion::async_recursion;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::time::Duration;
use wiwi::clock_timer_2::chrono::Local;
use wiwi::prelude::*;

struct Event<'a> {
	release_hour: u32,
	release_minute: u32,
	chapter_id: Option<u32>,
	description: Option<&'a str>,
	short_description: Option<&'a str>,
}

const EVENTS: &[Event] = &[
	Event {
		release_hour: 0,
		release_minute: 30,
		chapter_id: Some(1738301),
		description: Some(
			"I agree you, Pinkie is super cute!\n\nI love to give Pinkie lots of hugs!",
		),
		short_description: Some("Pinkie is cute!"),
	},
	Event {
		release_hour: 1,
		release_minute: 0,
		chapter_id: Some(1738302),
		description: Some(
			"I agree you, Fluttershy is super cute!\n\nI love to give Fluttershy lots of hugs!",
		),
		short_description: Some("Fluttershy is cute!"),
	},
	Event {
		release_hour: 1,
		release_minute: 30,
		chapter_id: Some(1738303),
		description: Some(
			"I agree you, Rarity is super cute!\n\nI love to give Rarity lots of hugs!",
		),
		short_description: None,
	},
	Event {
		release_hour: 2,
		release_minute: 0,
		chapter_id: Some(1738304),
		description: None,
		short_description: Some("Twilight Sparkle is cute!"),
	},
	Event {
		release_hour: 2,
		release_minute: 30,
		chapter_id: None,
		description: None,
		short_description: Some("Rainbow Dash is cute!"),
	},
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let story_id = 552650;
	let stories_domain = "https://www.fimfiction.net/api/v2/stories";
	let chapters_domain = "https://www.fimfiction.net/api/v2/chapters";

	let token = &env::args().collect::<Vec<_>>()[1];
	let client = Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

	let stories_url = format!("{stories_domain}/{story_id}");

	let mut timer = ClockTimer::builder()
		.with_start_datetime(Local::now())
		.with_duration(TimeDelta::try_hours(3).unwrap())
		.with_interval(TimeDelta::try_minutes(10).unwrap())
		.build();

	while let Some(tick) = timer.tick().await {
		let elapsed = tick.elapsed();
		let remaining = tick.remaining();
		let title = format!(
			"This Story will Explode in {} Hours and {} Minutes",
			remaining.num_hours(),
			remaining.num_minutes() - (elapsed.num_hours() * 60)
		);
		let events = EVENTS
			.iter()
			.filter(|event| {
				event.release_hour == elapsed.num_hours() as u32
					&& event.release_minute
						== (elapsed.num_minutes() - (elapsed.num_hours() * 60)) as u32
			})
			.collect::<Vec<_>>();
		if !events.is_empty() {
			for event in events {
				let story_json = story_json(
					story_id,
					title.clone(),
					event.description,
					event.short_description,
				)
				.to_string();
				let _ = send_api_request(&client, &headers, &stories_url, story_json, 0).await;
				if event.chapter_id.is_some() {
					let chapters_url = format!("{chapters_domain}/{}", event.chapter_id.unwrap());
					let chapter_json = chapter_json(event.chapter_id.unwrap()).to_string();
					let _ =
						send_api_request(&client, &headers, &chapters_url, chapter_json, 0).await;
				}
			}
		} else {
			let story_json = story_json(story_id, title, None, None).to_string();
			let _ = send_api_request(&client, &headers, &stories_url, story_json, 0).await;
		}
	}

	Ok(())
}

#[async_recursion]
async fn send_api_request(
	client: &Client, headers: &HeaderMap, url: &String, body: String, recursion_level: u32,
) -> Result<(), Box<dyn std::error::Error>> {
	let api_response = client
		.patch(url)
		.headers(headers.clone())
		.body(body.clone())
		.send()
		.await?;
	if !api_response.status().is_success() {
		if recursion_level > 4 {
			eprintln!("Failed to send API request five times!\n{url}\n{body}");
			return Ok(());
		}
		tokio::time::sleep(Duration::from_millis(5000)).await;
		return send_api_request(client, headers, url, body, recursion_level + 1).await;
	}
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
	id: u32, title: String, description: Option<&str>, short_description: Option<&str>,
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
