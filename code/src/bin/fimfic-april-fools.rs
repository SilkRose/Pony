use async_recursion::async_recursion;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Duration;
use tokio::process::Command;
use wiwi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Event {
	release_hour: u32,
	release_minute: u32,
	title: Option<String>,
	cover: Option<String>,
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
	countdown_duration_hours: i64,
	covers_dir: String,
	cover_mane_js: String,
	fimfic_cookie_json: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Urls {
	story: String,
	chapter: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// 0 - ./fimfic-april-fools
	// 1 - api-token
	// 2 - arguments.json
	// 3 - events.json

	let arguments = env::args().collect::<Vec<_>>();
	let api_token = arguments[1].clone();
	let args: Arguments = serde_json::from_str(&fs::read_to_string(&arguments[2])?)?;
	let events: Vec<Event> = serde_json::from_str(&fs::read_to_string(&arguments[3])?)?;

	let urls = Urls {
		story: format!(
			"https://www.fimfiction.net/api/v2/stories/{}",
			args.story_id
		),
		chapter: "https://www.fimfiction.net/api/v2/chapters".to_string(),
	};

	let client = Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", api_token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

	let mut timer = ClockTimer::builder()
		.with_start_datetime(DateTime::from_timestamp(args.start_time, 0).unwrap())
		.with_duration(TimeDelta::try_hours(args.duration_hours).unwrap())
		.with_interval(TimeDelta::try_minutes(args.interval_minutes).unwrap())
		.build();

	while let Some(tick) = timer.tick().await {
		let countdown = tick.remaining().num_minutes()
			>= (args.duration_hours - args.countdown_duration_hours) * 60;
		handle_events(
			args.clone(),
			events.clone(),
			countdown,
			urls.clone(),
			client.clone(),
			headers.clone(),
			tick,
		)
		.await;
	}

	Ok(())
}

async fn handle_events(
	args: Arguments, events: Vec<Event>, countdown: bool, urls: Urls, client: Client,
	headers: HeaderMap, tick: Tick,
) {
	let elapsed = tick.elapsed();
	let remaining = tick.remaining();
	let remaining_hours =
		(remaining.num_minutes() / 60) - (args.duration_hours - args.countdown_duration_hours);
	let remaining_minutes = match remaining.num_minutes() % 60 == 0 {
		true => 0,
		false => {
			remaining.num_minutes()
				- ((args.duration_hours - args.countdown_duration_hours) * 60)
				- (remaining_hours * 60)
		}
	};
	let title = match countdown {
		true => Some(format!(
			"This Story will Explode in {:0>2}:{:0>2}",
			remaining_hours, remaining_minutes
		)),
		false => None,
	};
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
				args.story_id,
				&title.clone().or(event.title.clone()),
				&event.short_description,
				&event.description,
				&event.completion_status,
			);
			println!("{}", serde_json::to_string_pretty(&story_json).unwrap());
			let _ = send_api_request(&client, &headers, &urls.story, story_json, 0).await;
			if event.chapter_id.is_some() {
				let chapters_url = format!("{}/{}", urls.chapter, event.chapter_id.unwrap());
				let chapter_json = chapter_json(event.chapter_id.unwrap()).to_string();
				let _ = send_api_request(&client, &headers, &chapters_url, chapter_json, 0).await;
			}
			if event.cover.is_some() {
				let cover = format!("{}{}", args.covers_dir, event.cover.as_ref().unwrap());
				let command = format!(
					"node {} {} {} {}",
					args.cover_mane_js, args.story_id, cover, args.fimfic_cookie_json
				);

				#[cfg(target_os = "windows")]
				execute_windows_command_with_fail_msg(&command).await;

				#[cfg(not(target_os = "windows"))]
				execute_unix_command_with_fail_msg(&command).await;
			}
		}
	} else {
		if title.is_none() {
			return;
		}
		let story_json = story_json(args.story_id, &title, &None, &None, &None);
		println!("{}", serde_json::to_string_pretty(&story_json).unwrap());
		let _ = send_api_request(&client, &headers, &urls.story, story_json, 0).await;
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

async fn execute_windows_command_with_fail_msg(cmd: &str) {
	let output = Command::new("cmd")
		.args(["/C", cmd])
		.output()
		.await
		.unwrap();

	if !output.status.success() {
		println!("Failed to execute command: {cmd}")
	}
}

async fn execute_unix_command_with_fail_msg(cmd: &str) {
	let output = Command::new("sh")
		.arg("-c")
		.arg(cmd)
		.output()
		.await
		.unwrap();

	if !output.status.success() {
		println!("Failed to execute command: {cmd}")
	}
}
