#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { NodeHtmlMarkdown } from "node-html-markdown";
import * as plib from "./lib.ts";
import fs from "fs";

await mane();

async function mane() {
	const id = "237915";
	const user = "Silk+Rose";
	const blogs_url = "https://www.fimfiction.net/blog/";
	const user_blogs_url = `https://www.fimfiction.net/user/${id}/${user}/blog`;
	const blog_id_regex = /<a href="\/blog\/[0-9]+\/.*" >.*<\/a>/;
	let blog_ids = [];
	for (let i = 1; 1 < Infinity; i++) {
		const url = user_blogs_url + "?page=" + i;
		const html = await fetch(url).then((res) => res.text());
		if (!html.includes('<div class="blog_post_content bbcode">')) break;
		const ids = html
			.split("\n")
			.filter((l) => l.match(blog_id_regex))
			.map((b) => b.split("/")[2]);
		for (const id of ids) blog_ids.push(id);
	}
	for (const id of blog_ids) {
		const url = blogs_url + id;
		const html = (await fetch(url).then((res) => res.text()))
			.split("\n")
			.map((l) => l.trim());
		const main = html.indexOf('<div class="main">');
		const title = html[main - 5].split(/ >|<\/a>/)[1].replace("&#039;", "'");
		const time = html[main - 3].split(/data-time="|" title="/)[1];
		const blog = html[main + 2];
		const md = NodeHtmlMarkdown.translate(blog);
		const date = new Date(Number(time) * 1000);
		const year = date.getFullYear();
		const month = date.getMonth().toString().padStart(2, "0");
		const day = date.getDay().toString().padStart(2, "0");
		const hour = date.getHours().toString().padStart(2, "0");
		const minute = date.getMinutes().toString().padStart(2, "0");
		const second = date.getSeconds().toString().padStart(2, "0");
		const filename = `${day}-${hour}-${minute}-${second}.md`;
		const data =
			`# ${title}\n\n` +
			md
				.replaceAll(
					"https://static.fimfiction.net/images/emoticons",
					"../../ponies/emotes",
				)
				.replaceAll("(/", "(https://www.fimfiction.net/")
				.concat("\n");
		if (!fs.existsSync(`../blog/${year}/${month}/`)) {
			plib.mkDirs([`../blog/${year}/${month}`]);
		}
		const path = `../blog/${year}/${month}/${filename}`;
		plib.writeFile(path, data);
		console.log("Finished downloading blog: " + title);
	}
}
