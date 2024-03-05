#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { NodeHtmlMarkdown } from "node-html-markdown";
import * as plib from "./lib.ts";
import fs from "fs";

await mane();

async function mane() {
	const access_token = process.argv[2];
	const api_domain = "https://www.fimfiction.net/api/v2/stories";
	const stats_domain = "https://www.fimfiction.net/story/stats";
	for (let id = 551751; 1 < 552652; id++) {
		let api_status = 200;
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
		if (api_status === 429) {
			sleep(5000);
			id = id - 1;
			continue;
		}
		const stats_html = await fetch(`${stats_domain}/${id}`).then((res) =>
			res.text(),
		);
		console.log(api_json);
		if (!stats_html.includes('data-controller="story-stats"')) continue;
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
	}
}

function sleep(milliseconds: number) {
	const date = Date.now();
	let current_date = null;
	do {
		current_date = Date.now();
	} while (current_date - date < milliseconds);
}
