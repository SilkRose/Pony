use pony::command::execute_command;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::timeout;
use wiwi::prelude::*;

type Events = HashMap<String, Event>;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Event {
	Chapter(Chapter),
	VoteResult(VoteResult),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chapter {
	// ID of the next vote result event.
	next_event: Option<String>,
	// ID of the next chapter event.
	vote_result_event: Option<String>,
	// Duration of the event.
	duration: u32,
	// Cover file name, no path included.
	cover: Option<String>,
	// Title for setting when no vote is taking place.
	like_delta: i32,
	// Content to post.
	content: Option<String>,
	// The title for before the vote passes.
	title_below: String,
	// The title for when exact votes are met.
	title_exact: Option<String>,
	// The title for after the vote passes.
	title_above: Option<String>,
	// Long description.
	description: String,
	// The short description for before the vote passes.
	short_description_below: String,
	// The short description for when exact votes are met.
	short_description_exact: Option<String>,
	// The short description for after the vote passes.
	short_description_above: Option<String>,
	// Authors note.
	authors_note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VoteResult {
	// The title for if the vote fails.
	title_fail: String,
	// The title for if the vote passes.
	title_pass: String,
	// Short description for if the vote fails.
	short_description_fail: String,
	// Short description for if the vote passes.
	short_description_pass: String,
	// Content to pre-pend if the vote passes.
	content_pass: String,
	// Content to pre-pend if the vote fails.
	content_fail: String,
	// Replacements for if the vote passes.
	content_pass_replace: Option<HashMap<String, String>>,
	// Replacements for if the vote fails.
	content_fail_replace: Option<HashMap<String, String>>,
	// Authors note.
	authors_note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct EventState {
	// Likes at the start of the last vote.
	likes: u32,
	// Minutes into the current event.
	elapsed: u32,
	// Last event.
	chapter: Option<String>,
	// Replacements for passed votes.
	content_pass_replace: HashMap<String, String>,
	// Replacements for failed votes.
	content_fail_replace: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Arguments {
	story_id: u32,
	start_time: i64,
	skip_past_events: bool,
	duration_hours: i64,
	interval_minutes: i64,
	countdown_duration_hours: i64,
	covers_dir: String,
	content_dir: String,
	cover_mane_js: String,
	fimfic_cookie_json: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Urls {
	story: String,
	chapter: String,
}

#[derive(Debug, Clone)]
struct FimficRequest {
	client: Client,
	headers: HeaderMap,
	interval: Duration,
	interval_step: Duration,
	interval_max: Duration,
	timeout: Duration,
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

	// API and site request structs, client, headers, and time intervals.
	let api = FimficRequest {
		client: Client::new(),
		headers: setup_api_headers(&api_token)?,
		interval: Duration::from_millis(500),
		interval_step: Duration::from_secs(2),
		interval_max: Duration::from_secs(120),
		timeout: Duration::from_secs(10),
	};

	let mut timer = ClockTimer::builder()
		.with_start_datetime(DateTime::from_timestamp(args.start_time, 0).unwrap())
		.with_duration(TimeDelta::try_hours(args.duration_hours).unwrap())
		.with_interval(TimeDelta::try_minutes(args.interval_minutes).unwrap())
		.build();

	let mut current_state = EventState::default();
	while let Some(tick) = timer.tick().await {
		if args.skip_past_events && tick.past_due() {
			continue;
		}

		let elapsed = tick.elapsed();
		let remaining = tick.remaining();
	}

	Ok(())
}

fn setup_api_headers(token: &str) -> Result<HeaderMap, Box<dyn Error>> {
	let mut headers = HeaderMap::new();
	headers.insert(
		AUTHORIZATION,
		HeaderValue::from_str(&format!("Bearer {}", token))?,
	);
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
	Ok(headers)
}

macro_rules! api_request {
	($fun:ident, $method:ident) => {};
}

api_request!(api_post_request, post);
api_request!(api_patch_request, patch);

async fn api_get_request(
	request: &FimficRequest, url: &str,
) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
	let mut interval = request.interval;
	loop {
		let start_time = unix_time()?;
		let res = timeout(
			request.timeout,
			request
				.client
				.get(url)
				.headers(request.headers.clone())
				.send(),
		)
		.await;
		match res {
			Ok(Ok(response)) => {
				return Ok(response);
			}
			Ok(Err(error)) => {
				println!("Request failed: {error}");
			}
			Err(error) => {
				println!("Request timed out: {error}");
			}
		}
		sleep(start_time, interval).await?;
		interval = if interval < request.interval_max {
			interval + request.interval_step
		} else {
			request.interval_max
		};
	}
}

async fn sleep(
	start_time: u128, interval: Duration,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let current_time = unix_time()?;
	let elapsed_time = Duration::from_millis((current_time - start_time).try_into()?);
	if elapsed_time > interval {
		return Ok(());
	};
	tokio::time::sleep(interval - elapsed_time).await;
	Ok(())
}

fn unix_time() -> Result<u128, Box<dyn std::error::Error + Send + Sync>> {
	Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
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
