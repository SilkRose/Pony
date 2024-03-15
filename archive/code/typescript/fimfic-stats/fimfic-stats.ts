#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { Database } from "bun:sqlite";
import * as cheerio from "cheerio";
import * as sql from "./sql-patterns.ts";
import {
	Tag,
	id_schema,
	api_schema,
	stats_schema,
} from "./types-and-schema.ts";
import * as plib from "./lib.ts";
import fs from "fs";

const db = new Database("./fimfic-stats.db", { create: true });
db.prepare(sql.story_index_table).run();
db.prepare(sql.authors_table).run();
db.prepare(sql.stories_table).run();
db.prepare(sql.tags_table).run();
db.prepare(sql.tag_links_table).run();
db.prepare(sql.chapters_table).run();
db.prepare(sql.stats_table).run();
db.prepare(sql.referral_sites_table).run();
db.prepare(sql.referrals_table).run();

await mane();

async function mane() {
	const version = 1;

	// API Bearer token is required to scrape the data.
	const access_token = process.argv[2];
	const api_domain = "https://www.fimfiction.net/api/v2/stories";
	const stats_domain = "https://www.fimfiction.net/story/stats";

	// Set a request interval to ensure API and HTTPS calls are rate limited.
	const request_interval = 1000;

	// Loop over IDs to scrape data.
	for (let id = 1; id <= 2000; id++) {
		const start_time = Date.now();
		let status = "unknown";

		// Set API and HTML status to -1.
		let api_status = -1;
		let html_status = -1;

		// Get data from the API.
		const api_json = await fetch(`${api_domain}/${id}`, {
			method: "GET",
			headers: {
				Authorization: `Bearer ${access_token}`,
				"Content-Type": "application/json",
			},
		}).then((response) => {
			api_status = response.status;
			return response.json();
		});

		// Get html of the stats page.
		const stats_html = await fetch(`${stats_domain}/${id}`).then((response) => {
			html_status = response.status;
			return response.text();
		});

		// Checks to see if the story is deleted or unpublished.
		if (api_status === 200 && html_status === 200) {
			status = "published";
		} else if (api_status === 404 && html_status === 404) {
			status = "deleted";
		} else if (api_status === 404 && html_status === 200) {
			status = "unpublished";
		}

		console.log(`${id}: ${status}`);
		const table = sql.insert_story_index(id, status, version, start_time);
		db.query(table).run();

		if (status != "published") {
			await sleep(start_time, Date.now(), request_interval);
			continue;
		}

		const api = api_schema.parse(api_json);
		//console.dir(api, { depth: null });

		db.query(
			sql.insert_author(
				Number(api.data.relationships.author.data.id),
				format_quote_string(api.included[0].attributes.name),
				new Date(api.included[0].attributes.date_joined).getTime() / 1000,
				api.included[0].attributes.num_followers,
				api.included[0].attributes.num_blog_posts,
			),
		).run();

		// Load the HTML with Cheerio.
		const document = cheerio.load(stats_html);

		// Get the tag IDs and names.
		let tags: Tag[] = [];
		document("ul.story-tags li").each((index, listItem) => {
			const tag = document(listItem).find("a");
			tags.push({
				id: Number(tag.attr("tag-id")),
				title: tag.attr("title")!,
				type: tag.attr("class")!,
				href: tag.attr("href")!,
				text: tag.text(),
			});
		});

		// Format the historical data into JSON.
		const data = document(".layout-two-columns[data-data]").attr("data-data")!;
		const stats = stats_schema.parse(JSON.parse(data));

		// Get the ranking and word count rankings from the HTML.
		const rankings = document('h1:contains("Rankings")').next("ul").find("li");
		const ranking = Number(document(rankings[0]).text().replace(/\D/g, ""));
		const word_ranking = Number(
			document(rankings[1]).text().replace(/\D/g, ""),
		);

		// Get the number of bookshelves and tracking from the HTML.
		const books = document('h1:contains("Bookshelves")').next("ul").find("li");
		const bookshelves = Number(document(books[0]).text().replace(/\D/g, ""));
		const tracking = Number(document(books[1]).text().replace(/\D/g, ""));

		// Get the number of referrals from each site from the HTML.
		let referrals: Record<string, number> = {};

		document('h1:contains("Referrals")')
			.next("ul")
			.find("li")
			.each(function () {
				const [site, count] = document(this).text().split(": ");
				referrals[site] = Number(count.replace(/\D/g, ""));
			});

		db.query(
			sql.insert_story(
				Number(api.data.id),
				format_quote_string(api.data.attributes.title),
				new Date(api.data.attributes.date_modified).getTime() / 1000,
				new Date(api.data.attributes.date_updated).getTime() / 1000,
				new Date(api.data.attributes.date_published).getTime() / 1000,
				!!api.data.attributes.cover_image ? 1 : 0,
				api.data.attributes.color.hex,
				api.data.attributes.num_views,
				api.data.attributes.total_num_views,
				api.data.attributes.num_comments,
				api.data.attributes.rating,
				api.data.attributes.completion_status,
				api.data.attributes.content_rating,
				api.data.attributes.num_likes,
				api.data.attributes.num_dislikes,
				ranking,
				word_ranking,
				bookshelves,
				tracking,
				Number(api.data.relationships.author.data.id),
				!!api.data.relationships.prequel
					? Number(api.data.relationships.prequel.data.id)
					: "NULL",
			),
		).run();

		tags.forEach((tag) => {
			db.query(
				sql.insert_tag(
					tag.id,
					format_quote_string(tag.title),
					format_quote_string(tag.type.replace("tag-", "")),
					format_quote_string(tag.text),
					format_quote_string(tag.href.replace("/tag/", "")),
				),
			).run();
			db.query(sql.insert_tag_link(id, tag.id)).run();
		});

		stats.chapters.forEach((chapter) => {
			db.query(
				sql.insert_chapter(
					id,
					chapter.chapter_num,
					format_quote_string(chapter.title),
					new Date(chapter.date).getTime() / 1000,
					Number(chapter.views),
					Number(chapter.words),
				),
			).run();
		});

		stats.stats.data.forEach((day) => {
			db.query(
				sql.insert_stats(
					id,
					new Date(day.date).getTime() / 1000,
					!!day.views ? day.views : "NULL",
					!!day.likes ? day.likes : "NULL",
					!!day.dislikes ? day.dislikes : "NULL",
				),
			).run();
		});

		for (const site in referrals) {
			db.query(sql.insert_referral_site(site)).run();
			const site_id_json = db.query(sql.retrieve_referral_site_id(site)).get();
			const site_id = id_schema.parse(site_id_json).id;
			db.query(sql.insert_referral(id, site_id, referrals[site])).run();
		}

		await sleep(start_time, Date.now(), request_interval);
	}
}

function sleep(
	start_time: number,
	current_time: number,
	interval: number,
): Promise<void> {
	const elapsed_time = current_time - start_time;
	console.log(elapsed_time);
	if (elapsed_time > interval) return Promise.resolve();
	const remaining_time = interval - elapsed_time;
	return new Promise((res) => setTimeout(res, remaining_time));
}

function format_quote_string(text: string): string {
	return text.replaceAll("\n", "\n\n").replaceAll("'", "''");
}
