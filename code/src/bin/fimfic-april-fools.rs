use async_recursion::async_recursion;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::time::Duration;
use wiwi::clock_timer_2::chrono::Local;
use wiwi::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Event {
	release_hour: u32,
	release_minute: u32,
	title: Option<String>,
	chapter_id: Option<u32>,
	description: Option<String>,
	short_description: Option<String>,
	completion_status: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// ./fimfic-april-fools json-file-path api-token
	// planeed: ./fimfic-april-fools story-id api-token json-file-path fimfic-cover-mane.js
	let events: Vec<Event> =
		serde_json::from_str(&fs::read_to_string(&env::args().collect::<Vec<_>>()[1]).unwrap())
			.unwrap();
	let token = &env::args().collect::<Vec<_>>()[2];

	let story_id = 552650;
	let stories_domain = "https://www.fimfiction.net/api/v2/stories";
	let chapters_domain = "https://www.fimfiction.net/api/v2/chapters";

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
		.with_duration(TimeDelta::try_hours(1).unwrap())
		.with_interval(TimeDelta::try_minutes(1).unwrap())
		.build();

	while let Some(tick) = timer.tick().await {
		let elapsed = tick.elapsed();
		let remaining = tick.remaining();
		let title = format!(
			"This Story will Explode in {}:{:0>2}!",
			remaining.num_hours(),
			(remaining.num_minutes() - (remaining.num_hours() * 60))
		);
		let events = events
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
					&Some(title.clone()),
					&event.description,
					&event.short_description,
					&event.completion_status,
				);
				println!("{}", serde_json::to_string_pretty(&story_json).unwrap());
				let _ = send_api_request(&client, &headers, &stories_url, story_json, 0).await;
				if event.chapter_id.is_some() {
					let chapters_url = format!("{chapters_domain}/{}", event.chapter_id.unwrap());
					let chapter_json = chapter_json(event.chapter_id.unwrap()).to_string();
					let _ =
						send_api_request(&client, &headers, &chapters_url, chapter_json, 0).await;
				}
			}
		} else {
			let story_json = story_json(story_id, &Some(title), &None, &None, &None);
			println!("{}", serde_json::to_string_pretty(&story_json).unwrap());
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
	id: u32, title: &Option<String>, description: &Option<String>,
	short_description: &Option<String>, completion_status: &Option<String>,
) -> String {
	let mut json = format!("{{\"data\":{{\"id\":{},\"attributes\":{{", id);
	if let Some(name) = title {
		json.push_str(&format!("\"title\":\"{name}\","));
	}
	if let Some(desc) = description {
		json.push_str(&format!("\"description\":\"{desc}\","));
	}
	if let Some(desc) = short_description {
		json.push_str(&format!("\"short_description\":\"{desc}\","));
	}
	if let Some(status) = completion_status {
		json.push_str(&format!("\"completion_status\":\"{status}\","));
	}
	json = json.trim_end_matches(',').to_string();
	json.push_str("}}}");
	json
}
