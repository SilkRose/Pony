#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import * as plib from "./lib.ts";
import fs from "fs";

await mane();

async function mane() {
	// API Bearer token is required to scrape the data.
	const access_token = process.argv[2];
	const api_domain = "https://www.fimfiction.net/api/v2/stories";
	const stats_domain = "https://www.fimfiction.net/story/stats";
	// Loop over IDs to scrape data.
	for (let id = 551751; id < 552652; id++) {
		// Set API status to 200.
		let api_status = 200;
		// Get data from the API.
		const api_json = await fetch(`${api_domain}/${id}`, {
			method: "GET",
			headers: {
				Authorization: `Bearer ${access_token}`,
				"Content-Type": "application/json",
			},
		}).then((response) => {
			if (!response.ok) {
				api_status = response.status;
				console.error(`HTTP error! Status: ${response.status}`);
			}
			return response.json();
		});
		// Check for rate limiting.
		if (api_status === 429) {
			sleep(5000);
			id = id - 1;
			continue;
		}
		console.log(api_json);
		// Get html of the stats page.
		const stats_html = await fetch(`${stats_domain}/${id}`).then((res) =>
			res.text(),
		);
		// Checks to see if the story is deleted or unpublished.
		if (!stats_html.includes('data-controller="story-stats"') && api_status === 404) {
			console.warn("deleted story")
		}
		if (stats_html.includes('data-controller="story-stats"') && api_status === 404) {
			console.warn("unpublished story")
		}
		if (!stats_html.includes('data-controller="story-stats"')) continue;
		// Format the historical data into JSON.
		const stats = stats_html
			.split("\n")
			.filter((l) =>
				l.startsWith('<div class="layout-two-columns story-stats"'),
			)[0]
			.replace(
				/^<div class="layout-two-columns story-stats" data-controller="story-stats" data-data="/,
				"",
			)
			.replace(/">$/, "")
			.replaceAll("&quot;", '"');
		
		console.log(id, JSON.parse(stats));
		sleep(1000);
	}
}

function sleep(milliseconds: number) {
	const date = Date.now();
	let current_date = null;
	do {
		current_date = Date.now();
	} while (current_date - date < milliseconds);
}
