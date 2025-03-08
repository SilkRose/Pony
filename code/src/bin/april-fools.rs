use pony::command::execute_command;
use pony::fimfiction_api::story::StoryApi;
use pony::time::format_milliseconds;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::timeout;
use wiwi::clock_timer::chrono::Utc;
use wiwi::prelude::*;

type Events = HashMap<u32, Chapter>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chapter {
	// ID of the next chapter event.
	next_event: Option<u32>,
	// Duration of the event.
	duration: i32,
	// Cover file name, no path included.
	cover: Option<String>,
	// Title for setting when no vote is taking place.
	like_delta: i32,
	// Title for the chapter.
	chapter_title: String,
	// Content to post.
	content: Option<String>,
	// The title for before the vote passes.
	title_below: String,
	// The title for when exact votes are met.
	title_exact: String,
	// The title for after the vote passes.
	title_above: String,
	// Long description.
	description: String,
	// The short description for before the vote passes.
	short_description_below: String,
	// The short description for when exact votes are met.
	short_description_exact: String,
	// The short description for after the vote passes.
	short_description_above: String,
	// Authors note.
	authors_note: Option<String>,
	// Vote result object.
	result: VoteResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StoryData {
	title: String,
	description: String,
	short_description: String,
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
	// Title for the chapter.
	chapter_title: String,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VoteOutcome {
	title: String,
	short_description: String,
	content: String,
	content_replace: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct EventState {
	// Likes at the start of the last vote.
	likes: i32,
	// Minutes into the current event.
	elapsed: i32,
	// Last event.
	chapter: u32,
	// The outcome of the vote.
	outcome: Option<VoteOutcome>,
	// Replacements for votes.
	content_replace: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Replacements {
	// The like delta between the target and recieved amounts.
	like_diff: i32,
	// The likes recieved since the start of the chapter.
	like_rec: i32,
	// The total likes on the story.
	like_total: i32,
	// The delta time remaining for the current chapter.
	minutes_left: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Arguments {
	story_id: u32,
	start_time: i64,
	skip_past_events: bool,
	duration_hours: i64,
	interval_minutes: i64,
	result_duration: i32,
	covers_dir: String,
	content_dir: String,
	cover_mane_js: String,
	events_json: String,
	event_state_json: String,
	fimfic_cookie_json: String,
	api_responses_json: String,
}

#[derive(Debug, Clone)]
struct FimficRequest {
	client: Client,
	headers: HeaderMap,
	interval: Duration,
	interval_step: Duration,
	interval_max: Duration,
	timeout: Duration,
	max_tries: u32,
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
	let events: Events = serde_json::from_str(&fs::read_to_string(&args.events_json)?)?;
	let mut state: EventState =
		serde_json::from_str(&fs::read_to_string(&args.event_state_json).unwrap_or_default())
			.unwrap_or_default();
	let mut responses: HashMap<u128, StoryApi<i32>> =
		serde_json::from_str(&fs::read_to_string(&args.api_responses_json).unwrap_or_default())
			.unwrap_or_default();

	let story_url = format!(
		"https://www.fimfiction.net/api/v2/stories/{}",
		args.story_id
	);
	let chapter_url = format!("{story_url}/chapters");

	// API and site request structs, client, headers, and time intervals.
	let api = FimficRequest {
		client: Client::new(),
		headers: setup_api_headers(&api_token)?,
		interval: Duration::from_millis(500),
		interval_step: Duration::from_secs(2),
		interval_max: Duration::from_secs(120),
		timeout: Duration::from_secs(10),
		max_tries: 4,
	};

	let program_start = Utc::now();
	let start_time = DateTime::from_timestamp(args.start_time, 0).unwrap();
	let duration = TimeDelta::try_hours(args.duration_hours).unwrap();
	let interval = TimeDelta::try_minutes(args.interval_minutes).unwrap();
	let end_time = start_time.checked_add_signed(duration).unwrap();
	let program_start_utc = program_start.format("%Y-%m-%d %H:%M:%S").to_string();
	let start_utc = start_time.format("%Y-%m-%d %H:%M:%S").to_string();
	let start_diff = program_start.timestamp_millis() - start_time.timestamp_millis();

	let mut timer = ClockTimer::builder()
		.with_start_datetime(start_time)
		.with_duration(duration)
		.with_interval(interval)
		.build();

	if program_start < start_time {
		println!(
			"{program_start_utc}: event will start in {} at {start_utc}",
			format_milliseconds(start_diff.unsigned_abs() as u128, None)?
		);
	} else if program_start > start_time && program_start < end_time {
		println!(
			"{program_start_utc}: event started {} ago, at {start_utc}",
			format_milliseconds(start_diff.unsigned_abs() as u128, None)?
		);
	};

	while let Some(tick) = timer.tick().await {
		if args.skip_past_events && tick.past_due() {
			continue;
		}
		let time = unix_time()?;
		let current_event = events
			.get(&state.chapter)
			.expect("Event should be present.");
		let response = api_get_request(&api, &story_url).await?;
		let story = response.json::<StoryApi<i32>>().await?;
		responses.insert(time, story.clone());
		fs::write(
			args.api_responses_json.clone(),
			serde_json::to_string(&responses)?,
		)?;

		let time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

		// We use likes for release mode.
		#[cfg(not(debug_assertions))]
		let metric = story.data.attributes.num_likes;

		// We use comments for debug mode.
		#[cfg(debug_assertions)]
		let metric = story.data.attributes.num_comments;

		let replace = Replacements {
			like_diff: current_event.like_delta - (metric - state.likes),
			like_rec: metric - state.likes,
			like_total: metric,
			minutes_left: (current_event.duration - state.elapsed - args.result_duration).abs(),
		};

		let mut changes: Vec<&str> = Vec::new();
		if state.elapsed < current_event.duration - args.result_duration {
			if state.elapsed == 0 {
				state.likes = metric;
				if let Some(ref cover) = current_event.cover {
					let cover = format!("{}{}", args.covers_dir, cover);
					let command = format!(
						r#"node "{}" {} "{}" "{}""#,
						args.cover_mane_js, args.story_id, cover, args.fimfic_cookie_json
					);
					changes.push("cover update");
					execute_command(&command).unwrap();
				}
				if let Some(path) = &current_event.content {
					let mut content = fs::read_to_string(format!("{}{path}", args.content_dir))?;
					for (hash, value) in &state.content_replace {
						content = content.replace(hash, value);
					}
					let chapter = chapter_json(
						&current_event.chapter_title,
						&content,
						current_event.authors_note.as_deref(),
					);
					changes.push("init chapter post");
					api_post_request(&api, chapter.to_string(), &chapter_url).await?;
				}
			}
			let update = story_parameters(current_event.clone(), metric, state.likes);
			let json = story_json(
				args.story_id,
				&replace_text(&update.title, &replace),
				&replace_text(&update.short_description, &replace),
				&replace_text(&update.description, &replace),
			);
			changes.push("story update");
			let response = api_patch_request(&api, json, &story_url).await?;
			let _ = response.json::<StoryApi<u32>>().await?;
		} else if state.elapsed >= current_event.duration - args.result_duration
			&& state.elapsed < current_event.duration
		{
			if state.elapsed == current_event.duration - args.result_duration {
				let passed = metric >= state.likes + current_event.like_delta;
				let result = vote_results(current_event.result.clone(), passed);
				let mut content =
					fs::read_to_string(format!("{}{}", args.content_dir, result.content))?;
				if let Some(replacements) = result.content_replace.clone() {
					state.content_replace.extend(replacements);
				}
				for (hash, value) in &state.content_replace {
					content = content.replace(hash, value);
				}
				let chapter = chapter_json(
					&current_event.result.chapter_title,
					&content,
					current_event.result.authors_note.as_deref(),
				);
				changes.push("result chapter post");
				api_post_request(&api, chapter.to_string(), &chapter_url).await?;
				state.outcome = Some(result);
			}
			let outcome = state
				.outcome
				.clone()
				.expect("Outcome should always be present.");
			let json = story_json(
				args.story_id,
				&replace_text(&outcome.title, &replace),
				&replace_text(&outcome.short_description, &replace),
				&format!(
					"{}\n\n[hr]\n\n{}",
					replace_text(&outcome.short_description, &replace),
					replace_text(&current_event.description, &replace)
				),
			);
			changes.push("result story update");
			let response = api_patch_request(&api, json, &story_url).await?;
			let _ = response.json::<StoryApi<u32>>().await?;
		}
		state.elapsed += 1;
		fs::write(
			args.event_state_json.clone(),
			serde_json::to_string(&state)?,
		)?;
		println!(
			"{time} - diff: {:0>2}, rec: {:0>2}, total: {:0>3}, mins left: {:0>2}",
			replace.like_diff, replace.like_rec, replace.like_total, replace.minutes_left
		);
		println!("{}", changes.join(", "));
		if state.elapsed == current_event.duration {
			if let Some(next) = current_event.next_event {
				state.outcome = None;
				state.chapter = next;
				state.elapsed = 0;
				continue;
			}
			// If we finish, set chapter to MAX to error on restart.
			state.chapter = u32::MAX;
			let time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
			let runtime = program_start.timestamp_millis() - Utc::now().timestamp_millis();
			println!(
				"{time}: event completed with a runtime of {}",
				format_milliseconds(runtime as u128, None)?
			);
			exit(0);
		};
	}

	Ok(())
}

fn story_parameters(chapter: Chapter, total_likes: i32, starting_likes: i32) -> StoryData {
	match (starting_likes + chapter.like_delta).cmp(&total_likes) {
		Ordering::Less => StoryData {
			title: chapter.title_below,
			description: format!(
				"{}\n\n[hr]\n\n{}",
				chapter.short_description_below, chapter.description
			),
			short_description: chapter.short_description_below,
		},
		Ordering::Equal => StoryData {
			title: chapter.title_exact,
			description: format!(
				"{}\n\n[hr]\n\n{}",
				chapter.short_description_exact, chapter.description
			),
			short_description: chapter.short_description_exact,
		},
		Ordering::Greater => StoryData {
			title: chapter.title_above,
			description: format!(
				"{}\n\n[hr]\n\n{}",
				chapter.short_description_above, chapter.description
			),
			short_description: chapter.short_description_above,
		},
	}
}

fn vote_results(options: VoteResult, passed: bool) -> VoteOutcome {
	match passed {
		true => VoteOutcome {
			title: options.title_pass,
			short_description: options.short_description_pass,
			content: options.content_pass,
			content_replace: options.content_pass_replace,
		},
		false => VoteOutcome {
			title: options.title_fail,
			short_description: options.short_description_fail,
			content: options.content_fail,
			content_replace: options.content_fail_replace,
		},
	}
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
	($fun:ident, $method:ident) => {
		async fn $fun(
			request: &FimficRequest, body: String, url: &str,
		) -> Result<Response, Box<dyn std::error::Error>> {
			let mut interval = request.interval;
			let mut tries = 1;
			loop {
				let start_time = unix_time()?;
				let res = timeout(
					request.timeout,
					request
						.client
						.$method(url)
						.body(body.clone())
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
				if tries > request.max_tries {
					return Err("Max tries reached!".into());
				}
				tries += 1;
			}
		}
	};
}

api_request!(api_post_request, post);
api_request!(api_patch_request, patch);

async fn api_get_request(
	request: &FimficRequest, url: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
	let mut interval = request.interval;
	let mut tries = 1;
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
		if tries > request.max_tries {
			return Err("Max tries reached!".into());
		}
		tries += 1;
	}
}

async fn sleep(start_time: u128, interval: Duration) -> Result<(), Box<dyn std::error::Error>> {
	let current_time = unix_time()?;
	let elapsed_time = Duration::from_millis((current_time - start_time).try_into()?);
	if elapsed_time > interval {
		return Ok(());
	};
	tokio::time::sleep(interval - elapsed_time).await;
	Ok(())
}

fn unix_time() -> Result<u128, Box<dyn std::error::Error>> {
	Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

fn chapter_json(title: &str, content: &str, authors_note: Option<&str>) -> Value {
	json!({
		 "data": {
			  "type": "chapter",
			  "attributes": {
					"title": title,
					"content": content,
					"authors_note": authors_note.unwrap_or_default(),
					"published": true
			  }
		 }
	})
}

fn story_json(id: u32, title: &str, short_description: &str, description: &str) -> String {
	let mut attributes = HashMap::new();
	attributes.insert("title", title);
	attributes.insert("short_description", short_description);
	attributes.insert("description", description);
	let json = json!({
		"data": {
			"id": id,
			"attributes": serde_json::to_value(attributes).unwrap()
		}
	});
	serde_json::to_string(&json).unwrap()
}

fn replace_text(text: &str, replace: &Replacements) -> String {
	let mut result = String::new();
	let tokens = text.split('%');
	for token in tokens {
		if token.starts_with("ld[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.like_diff));
		} else if token.starts_with("lr[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.like_rec));
		} else if token.starts_with("lt[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.like_total));
		} else if token.starts_with("ml[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.minutes_left));
		} else if token == "ld" {
			result.push_str(&replace.like_diff.to_string());
		} else if token == "lr" {
			result.push_str(&replace.like_rec.to_string());
		} else if token == "lt" {
			result.push_str(&replace.like_total.to_string());
		} else if token == "ml" {
			result.push_str(&replace.minutes_left.to_string());
		} else {
			result.push_str(token);
		}
	}
	result
}

fn split_count(token: &str, count: i32) -> String {
	let (plural, single) = token.split_once('|').expect("Should always be present.");
	if count == 1 {
		single.trim_end_matches(']').to_string()
	} else {
		plural.split_once('[').unwrap().1.into()
	}
}
