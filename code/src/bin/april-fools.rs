use chrono_tz::America::New_York;
use pony::command::execute_command;
use pony::fimfiction_api::fimfic_api_headers;
use pony::fimfiction_api::story::StoryApi;
use pony::http::{Request, api_get_request, api_patch_request, api_post_request};
use pony::markdown;
use pony::time::{format_milliseconds, unix_time};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Duration;
use wiwi::clock_timer::chrono::Utc;
use wiwi::prelude::*;

type Events = HashMap<u32, ChapterEvent>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ChapterEvent {
	// Duration of the event.
	duration: i32,
	// Cover file name, no path included.
	cover: Option<String>,
	// Title for setting when no vote is taking place.
	like_delta: i32,
	// The chapter id.
	chapter_id: u32,
	// Title for the chapter.
	chapter_title: Option<String>,
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
	result: ResultEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StoryData {
	// New title.
	title: String,
	// New long description.
	description: String,
	// New short description.
	short_description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ResultEvent {
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
	// ID of the next chapter event if the vote passes.
	next_event_fail: Option<u32>,
	// ID of the next chapter event if the vote fails.
	next_event_pass: Option<u32>,
	// Replacements for if the vote passes.
	content_pass_replace: Option<HashMap<String, String>>,
	// Replacements for if the vote fails.
	content_fail_replace: Option<HashMap<String, String>>,
	// Authors note.
	authors_note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ResultData {
	// Chapter title.
	title: String,
	// New short description.
	short_description: String,
	// Chapter content to post.
	content: String,
	// Repalcements to apply to all future chapters.
	content_replace: Option<HashMap<String, String>>,
	// ID of the next event.
	next_event: Option<u32>,
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
	outcome: Option<ResultData>,
	// Replacements for votes.
	content_replace: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Replacements {
	// The chapter id.
	chapter_id: u32,
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
	// The story ID to update.
	story_id: u32,
	// The unix time in seconds to start at.
	start_time: i64,
	// Minutes a result chapter should be posted before the end of an event.
	result_duration: i32,
	// Covers directory.
	covers_dir: String,
	// Content directory.
	content_dir: String,
	// Cover mane.js file path.
	cover_mane_js: String,
	// Events json file path.
	events_json: String,
	// State json file path.
	event_state_json: String,
	// FIMFiction cookie.json file path.
	fimfic_cookie_json: String,
	// Response json file path.
	api_responses_json: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// 0 - ./april-fools
	// 1 - api-token
	// 2 - arguments.json

	// Variable setup.
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

	// Check the arugments and events are correct and all files and folders exist.
	check_arguments(&args)?;
	let checked = check_events(&events, &args, state.chapter)?;
	let missing = events
		.clone()
		.into_iter()
		.filter(|event| checked.contains(event))
		.collect::<Events>();
	let missing = missing
		.into_iter()
		.map(|e| e.0.to_string())
		.collect::<Vec<_>>();
	println!("Missing events: {}", missing.join(", "));

	// URL setup.
	let story_url = format!(
		"https://www.fimfiction.net/api/v2/stories/{}",
		args.story_id
	);
	let chapter_url = format!("{story_url}/chapters");

	// API request structs, client, headers, and time intervals.
	let api = Request {
		client: Client::new(),
		headers: fimfic_api_headers(None, &api_token)?,
		interval: Duration::from_millis(500),
		interval_step: Duration::from_secs(2),
		interval_max: Duration::from_secs(120),
		timeout: Duration::from_secs(10),
		max_tries: 4,
	};

	// Time variables.
	let program_start = Utc::now();
	let start_time = DateTime::from_timestamp(args.start_time, 0).unwrap();
	// Event ends when all chapter events finish.
	let duration = TimeDelta::try_hours(1_000_000).unwrap();
	// Interval is always 1 minute.
	let interval = TimeDelta::try_minutes(1).unwrap();
	let program_start_utc = program_start.format("%Y-%m-%d %H:%M:%S").to_string();
	let start_utc = start_time.format("%Y-%m-%d %H:%M:%S").to_string();
	let start_diff = program_start.timestamp_millis() - start_time.timestamp_millis();

	// Timer setup.
	let mut timer = ClockTimer::builder()
		.with_start_datetime(start_time)
		.with_duration(duration)
		.with_interval(interval)
		.build();

	// Program start log.
	match program_start < start_time {
		true => println!(
			"{program_start_utc}: event will start in {} at {start_utc}",
			format_milliseconds(start_diff.unsigned_abs() as u128, None)?
		),
		false => println!(
			"{program_start_utc}: event started {} ago, at {start_utc}",
			format_milliseconds(start_diff.unsigned_abs() as u128, None)?
		),
	}

	// Mane loop.
	while let Some(tick) = timer.tick().await {
		// Should always skip past events.
		if tick.past_due() {
			continue;
		}
		// API story response get and save.
		let time = unix_time()?.as_millis();
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

		// Time for logging.
		let time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

		// We use likes for release mode.
		#[cfg(not(debug_assertions))]
		let metric = story.data.attributes.num_likes;

		// We use comments for debug mode.
		#[cfg(debug_assertions)]
		let metric = story.data.attributes.num_comments;

		// Replacements for title, short description, and description.
		let replace = Replacements {
			like_diff: current_event.like_delta - (metric - state.likes),
			like_rec: metric - state.likes,
			like_total: metric,
			chapter_id: current_event.chapter_id,
			minutes_left: (current_event.duration - state.elapsed - args.result_duration).abs(),
		};

		// Event ending time for author's note.
		let event_end = unix_time()? + Duration::from_secs(replace.minutes_left as u64 * 60);
		let end_time = DateTime::from_timestamp(event_end.as_secs() as i64, 0)
			.unwrap()
			.with_timezone(&New_York);
		let end_string = end_time.format("%I:%M %p").to_string();

		// Variable for keeping track of changes.
		let mut changes: Vec<&str> = Vec::new();
		// First if block for while elapsed is before the result chapter.
		if state.elapsed < current_event.duration - args.result_duration {
			// Check if 0 for posting a chapter and updating the cover.
			if state.elapsed == 0 {
				// Save likes for the rest of the chapter/
				state.likes = metric;
				// Update cover if set.
				if let Some(ref cover) = current_event.cover {
					let cover = format!("{}{}", args.covers_dir, cover);
					let command = format!(
						r#"node "{}" {} "{}" "{}""#,
						args.cover_mane_js, args.story_id, cover, args.fimfic_cookie_json
					);
					changes.push("cover update");
					execute_command(&command).unwrap();
				}
				// Post a chapter if set.
				if let Some(path) = &current_event.content {
					let mut content = fs::read_to_string(format!("{}{path}", args.content_dir))?;
					// Replace content based off previous votes.
					for (hash, value) in &state.content_replace {
						content = content.replace(hash, value);
					}
					// Replace ending time in author's note.
					let authors_note = current_event
						.authors_note
						.as_ref()
						.map(|text| text.replace("%ee%", &end_string))
						.map(|text| replace_text(&text, &replace));
					// Convert the content from markdown to nncode.
					content = markdown::bbcode::parse(&content, &markdown::WarningType::Warn);
					// Construct the JSON.
					let chapter = chapter_json(
						&current_event
							.chapter_title
							.clone()
							.expect("Title should be set if content is some."),
						&content,
						authors_note.as_deref(),
					);
					changes.push("init chapter post");
					// Send post request to FIMFiction.
					api_post_request(&api, chapter.to_string(), &chapter_url).await?;
				}
			}
			// Get correct story data based off vote count.
			let update = story_parameters(current_event.clone(), metric, state.likes);
			// Construct the JSON and replace variables in the data.
			let json = story_json(
				args.story_id,
				&replace_text(&update.title, &replace),
				&replace_text(&update.short_description, &replace),
				&replace_text(&update.description, &replace),
			);
			changes.push("story update");
			// Send patch request to FIMFiction.
			let response = api_patch_request(&api, json.to_string(), &story_url).await?;
			// Derserialize to confirm the changes worked.
			let _ = response.json::<StoryApi<u32>>().await?;
		} else if state.elapsed >= current_event.duration - args.result_duration
			&& state.elapsed < current_event.duration
		{
			// Post a result chapter if it's the first tick of the result.
			if state.elapsed == current_event.duration - args.result_duration {
				// Check if the vote passed.
				let passed = metric >= state.likes + current_event.like_delta;
				// Get results for the vote.
				let result = vote_results(current_event.result.clone(), passed);
				let mut content =
					fs::read_to_string(format!("{}{}", args.content_dir, result.content))?;
				// Extend stored replacements if they are set.
				if let Some(replacements) = result.content_replace.clone() {
					state.content_replace.extend(replacements);
				}
				// Replace content based off previous votes.
				for (hash, value) in &state.content_replace {
					content = content.replace(hash, value);
				}
				// Replace ending time in author's note.
				let authors_note = current_event
					.authors_note
					.as_ref()
					.map(|text| text.replace("%ee%", &end_string))
					.map(|text| replace_text(&text, &replace));
				// Convert the content from markdown to nncode.
				content = markdown::bbcode::parse(&content, &markdown::WarningType::Warn);
				// Construct the JSON.
				let chapter = chapter_json(
					&current_event.result.chapter_title,
					&content,
					authors_note.as_deref(),
				);
				changes.push("result chapter post");
				// Send post request to FIMFiction.
				api_post_request(&api, chapter.to_string(), &chapter_url).await?;
				// Save the outcome in the state variable.
				state.outcome = Some(result);
			}
			// Get outcome that we know is set.
			let outcome = state
				.outcome
				.clone()
				.expect("Outcome should always be present.");
			// Construct the JSON and replace variables in the data.
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
			// Send patch request to FIMFiction.
			let response = api_patch_request(&api, json.to_string(), &story_url).await?;
			// Derserialize to confirm the changes worked.
			let _ = response.json::<StoryApi<u32>>().await?;
		}
		// Increment the elapsed time.
		state.elapsed += 1;
		// Save changes to disk.
		fs::write(
			args.event_state_json.clone(),
			serde_json::to_string(&state)?,
		)?;
		// End of tick logging.
		println!(
			"{time} - diff: {:0>2}, rec: {:0>2}, total: {:0>3}, mins left: {:0>2}, end time: {} EST",
			replace.like_diff,
			replace.like_rec,
			replace.like_total,
			replace.minutes_left,
			end_string
		);
		// Change logging.
		println!("{}", changes.join(", "));
		// Handling for end of tick.
		if state.elapsed >= current_event.duration {
			if let Some(ref outcome) = state.outcome {
				if let Some(next) = outcome.next_event {
					state.outcome = None;
					state.chapter = next;
					state.elapsed = 0;
					continue;
				}
			}
			// If we finish, set chapter to MAX to error on restart.
			state.chapter = u32::MAX;
			// Save file if events are all over.
			fs::write(
				args.event_state_json.clone(),
				serde_json::to_string(&state)?,
			)?;
			// End of programm logging.
			let time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
			let runtime = program_start.timestamp_millis() - Utc::now().timestamp_millis();
			println!(
				"{time}: event completed with a runtime of {}",
				format_milliseconds(runtime as u128, None)?
			);
			break;
		};
	}
	Ok(())
}

fn story_parameters(chapter: ChapterEvent, total_likes: i32, starting_likes: i32) -> StoryData {
	// Get the story data for if likes is above, at, or below than the target.
	// \n\n[hr]\n\n is to format the long description correctly with the short description at the top.
	match (starting_likes + chapter.like_delta).cmp(&total_likes) {
		Ordering::Greater => StoryData {
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
		Ordering::Less => StoryData {
			title: chapter.title_above,
			description: format!(
				"{}\n\n[hr]\n\n{}",
				chapter.short_description_above, chapter.description
			),
			short_description: chapter.short_description_above,
		},
	}
}

fn vote_results(options: ResultEvent, passed: bool) -> ResultData {
	// Returns the data based off if the vote passed.
	match passed {
		true => ResultData {
			title: options.title_pass,
			short_description: options.short_description_pass,
			content: options.content_pass,
			content_replace: options.content_pass_replace,
			next_event: options.next_event_pass,
		},
		false => ResultData {
			title: options.title_fail,
			short_description: options.short_description_fail,
			content: options.content_fail,
			content_replace: options.content_fail_replace,
			next_event: options.next_event_fail,
		},
	}
}

fn chapter_json(title: &str, content: &str, authors_note: Option<&str>) -> Value {
	// Construct the json for chapters.
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

fn story_json(id: u32, title: &str, short_description: &str, description: &str) -> Value {
	// Construct the json for story updates.
	json!({
		"data": {
			"id": id,
			"attributes": {
				"title": title,
				"description": description,
				"short_description": short_description
			}
		}
	})
}

fn replace_text(text: &str, replace: &Replacements) -> String {
	// Variable setup.
	let mut result = String::new();
	let tokens = text.split('%');
	for token in tokens {
		// Check for plural | singular replacement.
		if token.starts_with("ld[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.like_diff));
		} else if token.starts_with("lr[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.like_rec));
		} else if token.starts_with("lt[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.like_total));
		} else if token.starts_with("ml[") && token.ends_with("]") {
			result.push_str(&split_count(token, replace.minutes_left));
		// Check for normal replacement.
		} else if token == "ld" {
			result.push_str(&replace.like_diff.to_string());
		} else if token == "lr" {
			result.push_str(&replace.like_rec.to_string());
		} else if token == "lt" {
			result.push_str(&replace.like_total.to_string());
		} else if token == "ml" {
			result.push_str(&replace.minutes_left.to_string());
		} else if token == "id" {
			result.push_str(&replace.chapter_id.to_string());
		} else {
			result.push_str(token);
		}
	}
	result
}

fn split_count(token: &str, count: i32) -> String {
	// Check and return based off if count is 1.
	let (plural, single) = token.split_once('|').expect("Should always be present.");
	if count == 1 {
		single.trim_end_matches(']').to_string()
	} else {
		plural.split_once('[').unwrap().1.into()
	}
}

fn check_arguments(args: &Arguments) -> Result<(), Box<dyn std::error::Error>> {
	let file_exists = fs::exists(&args.content_dir);
	if !file_exists? {
		return Err(format!("Folder missing at: {}", &args.content_dir).into());
	}
	let file_exists = fs::exists(&args.covers_dir);
	if !file_exists? {
		return Err(format!("Folder missing at: {}", &args.covers_dir).into());
	}
	let file_exists = fs::exists(&args.cover_mane_js);
	if !file_exists? {
		return Err(format!("File missing at: {}", &args.cover_mane_js).into());
	}
	let file_exists = fs::exists(&args.events_json);
	if !file_exists? {
		return Err(format!("File missing at: {}", &args.events_json).into());
	}
	let file_exists = fs::exists(&args.fimfic_cookie_json);
	if !file_exists? {
		return Err(format!("File missing at: {}", &args.fimfic_cookie_json).into());
	}
	Ok(())
}

fn check_events(
	events: &Events, args: &Arguments, id: u32,
) -> Result<Vec<(u32, ChapterEvent)>, Box<dyn std::error::Error>> {
	let mut checked: Vec<(u32, ChapterEvent)> = Vec::new();
	let event = events.get(&id);
	if event.is_none() {
		return Err(format!("Event data missing for ID: {id}").into());
	}
	let event = event.unwrap();
	if let Some(path) = &event.content {
		let path = format!("{}{path}", args.content_dir);
		let file_exists = fs::exists(&path);
		if !file_exists? {
			return Err(format!("File missing at: {path}").into());
		}
		if event.chapter_title.is_none() {
			return Err(format!("Chapter title missing for event ID: {id}").into());
		}
	}
	if let Some(cover) = &event.cover {
		let path = format!("{}{}", args.covers_dir, cover);
		let file_exists = fs::exists(&path);
		if !file_exists? {
			return Err(format!("File missing at: {path}").into());
		}
	}
	let result = event.result.clone();
	if result.next_event_pass.is_none() && result.next_event_fail.is_some() {
		return Err(format!("Pass event ID missing for next event ID: {id}").into());
	} else if result.next_event_pass.is_some() && result.next_event_fail.is_none() {
		return Err(format!("Fail event ID missing for next event ID: {id}").into());
	}
	let path = format!("{}{}", args.content_dir, result.content_pass);
	let file_exists = fs::exists(&path);
	if !file_exists? {
		return Err(format!("File missing at: {path}").into());
	}
	let path = format!("{}{}", args.content_dir, result.content_fail);
	let file_exists = fs::exists(&path);
	if !file_exists? {
		return Err(format!("File missing at: {path}").into());
	}
	if result.next_event_pass.is_some() && result.next_event_fail.is_some() {
		checked.extend(check_events(events, args, result.next_event_pass.unwrap())?);
		checked.extend(check_events(events, args, result.next_event_fail.unwrap())?);
	}
	checked.push((id, event.clone()));
	Ok(checked)
}
