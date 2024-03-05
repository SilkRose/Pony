#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { NodeHtmlMarkdown } from "node-html-markdown";
import * as plib from "./lib.ts";
import fs from "fs";

await mane();

async function mane() {
	const domain = "https://www.fimfiction.net/story/stats";
	for (let id = 551751; 1 < 552652; id++) {
		const url = `${domain}/${id}`;
		const html = await fetch(url).then((res) => res.text());
		if (!html.includes('data-controller="story-stats"')) continue;
		const stats = html
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
