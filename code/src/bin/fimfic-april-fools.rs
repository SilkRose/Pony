use async_recursion::async_recursion;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Duration;
use wiwi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Event {
	release_hour: u32,
	release_minute: u32,
	title: Option<String>,
	chapter_id: Option<u32>,
	description: Option<String>,
	short_description: Option<String>,
	completion_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Arguments {
	story_id: u32,
	start_time: i64,
	duration_hours: i64,
	interval_minutes: i64,
	extended_duration_hours: i64,
	extended_interval_minutes: i64,
	api_token: String,
	events: Vec<Event>,
	covers_dir: String,
	cover_mane_js: String,
	fimfic_cookie_json: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// ./fimfic-april-fools json-file-path api-token

	// 0  - ./fimfic-april-fools
	// 1  - story-id
	// 2  - start-unix-timestamp
	// 3  - duration in hours
	// 4  - interval in minutes
	// 5  - extended duration in hours
	// 6  - extended interval in minutes
	// 7  - api-token
	// 8  - events-file-path.json
	// 9  - covers-dir/
	// 10 - fimfic-cover-mane.js
	// 11 - fimfic-cookie.json

	let args = env::args().collect::<Vec<_>>();
	let args = Arguments {
		story_id: args[1].parse()?,
		start_time: args[2].parse()?,
		duration_hours: args[3].parse()?,
		interval_minutes: args[4].parse()?,
		extended_duration_hours: args[5].parse()?,
		extended_interval_minutes: args[6].parse()?,
		api_token: args[7].clone(),
		events: serde_json::from_str(&fs::read_to_string(&args[8])?)?,
		covers_dir: args[9].clone(),
		cover_mane_js: args[10].clone(),
		fimfic_cookie_json: args[11].clone(),
	};

	let stories_url = format!(
		"https://www.fimfiction.net/api/v2/stories/{}",
		args.story_id
	);
	let chapters_domain = "https://www.fimfiction.net/api/v2/chapters";

	let client = Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", args.api_token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

	let mut timer = ClockTimer::builder()
		.with_start_datetime(DateTime::from_timestamp(args.start_time, 0).unwrap())
		.with_duration(TimeDelta::try_hours(args.duration_hours).unwrap())
		.with_interval(TimeDelta::try_minutes(args.interval_minutes).unwrap())
		.build();

	while let Some(tick) = timer.tick().await {
		handle_events(
			args.clone(),
			true,
			&stories_url,
			chapters_domain,
			client.clone(),
			headers.clone(),
			tick,
		)
		.await;
	}

	timer = ClockTimer::builder()
		.with_start_datetime(DateTime::from_timestamp(args.start_time, 0).unwrap())
		.with_duration(TimeDelta::try_hours(args.extended_duration_hours).unwrap())
		.with_interval(TimeDelta::try_minutes(args.extended_interval_minutes).unwrap())
		.build();

	while let Some(tick) = timer.tick().await {
		handle_events(
			args.clone(),
			false,
			&stories_url,
			chapters_domain,
			client.clone(),
			headers.clone(),
			tick,
		)
		.await;
	}

	Ok(())
}

async fn handle_events(
	args: Arguments, countdown: bool, stories_url: &str, chapters_domain: &str, client: Client,
	headers: HeaderMap, tick: Tick,
) {
	let elapsed = tick.elapsed();
	let remaining = tick.remaining();
	let title = match countdown {
		true => Some(format!(
			"This Story will Explode in {}:{:0>2}!",
			remaining.num_hours(),
			(remaining.num_minutes() - (remaining.num_hours() * 60))
		)),
		false => None,
	};
	let events = args
		.events
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
				args.story_id,
				&title,
				&event.short_description,
				&event.description,
				&event.completion_status,
			);
			println!("{}", serde_json::to_string_pretty(&story_json).unwrap());
			let _ = send_api_request(&client, &headers, stories_url, story_json, 0).await;
			if event.chapter_id.is_some() {
				let chapters_url = format!("{chapters_domain}/{}", event.chapter_id.unwrap());
				let chapter_json = chapter_json(event.chapter_id.unwrap()).to_string();
				let _ = send_api_request(&client, &headers, &chapters_url, chapter_json, 0).await;
			}
		}
	} else {
		if title.is_none() {
			return;
		}
		let story_json = story_json(args.story_id, &title, &None, &None, &None);
		println!("{}", serde_json::to_string_pretty(&story_json).unwrap());
		let _ = send_api_request(&client, &headers, stories_url, story_json, 0).await;
	}
}

#[async_recursion]
async fn send_api_request(
	client: &Client, headers: &HeaderMap, url: &str, body: String, recursion_level: u32,
) -> Result<(), Box<dyn std::error::Error>> {
	let api_response = client
		.patch(url)
		.headers(headers.clone())
		.body(body.clone())
		.send()
		.await?;
	if !api_response.status().is_success() {
		println!("{:?}", api_response.text().await);
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
	id: u32, title: &Option<String>, short_description: &Option<String>,
	description: &Option<String>, completion_status: &Option<String>,
) -> String {
	let mut attributes = HashMap::new();
	if let Some(name) = title {
		attributes.insert("title", name);
	}
	if let Some(short_desc) = short_description {
		attributes.insert("short_description", short_desc);
	}
	if let Some(desc) = description {
		attributes.insert("description", desc);
	}
	if let Some(status) = completion_status {
		attributes.insert("completion_status", status);
	}
	let json = json!({
		"data": {
			"id": id,
			"attributes": serde_json::to_value(attributes).unwrap()
		}
	});
	serde_json::to_string(&json).unwrap()
}
