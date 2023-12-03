#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { repository } from "./package.json" assert { type: "json" };
import * as plib from "./lib.ts";
import path from "path";
import fs from "fs";

type Pony = {
	blogs: string;
	code: string;
	commits: string;
	covers: string;
	flash_fiction: string;
	ideas: string;
	names: string;
	size: string;
	stories: string;
	words: string;
};

type PonyCommit = {
	hash: string;
	subject: string;
	unix_time: number;
	code: number;
	code_change?: number;
	size: number;
	size_change?: number;
	words: number;
	word_change?: number;
};

await mane();

async function mane() {
	plib.checkInstalled(["git"]);
	plib.rmDirs(["./dist", "./pony-temp"]);
	plib.mkDirs(["./dist/api/v1"]);
	plib.writeFile("./dist/.nojekyll", "");
	plib.writeFile("./dist/CNAME", "pony.silkrose.dev");
	const git_url = repository.url.slice(4);
	plib.executeCommand(`git clone --quiet ${git_url} pony-temp`);
	process.chdir("./pony-temp");
	const git_log = plib
		.executeCommandReturn('git log mane --format="format:%H\n%s\n%ct\n"')
		.split("\n\n")
		.reverse();
	const pony: Pony = getPonyData(git_log.length);
	const pony_commits: PonyCommit[] = getChanges(getCommitData(git_log));
	const pony_string = plib.jsonFmt(JSON.stringify(pony));
	const pony_commits_string = plib.jsonFmt(JSON.stringify(pony_commits));
	plib.writeFile("../dist/api/v1/pony.json", pony_string + "\n");
	plib.writeFile(
		"../dist/api/v1/pony-commits.json",
		pony_commits_string + "\n",
	);
}

function getPonyData(commits: number) {
	return {
		blogs: countMarkdownFiles("./blogs"),
		code: countCode().toLocaleString("en-US"),
		commits: commits.toLocaleString("en-US"),
		covers: countCovers(),
		flash_fiction: countMarkdownFiles("./flash-fiction"),
		ideas: countFromFile("ideas.md", "## "),
		names: countFromFile("names.md", "- "),
		size: formatSize(countSize()),
		stories: countDirs("./stories"),
		words: countWords().toLocaleString("en-US"),
	};
}

function getCommitData(git_log: string[]) {
	return git_log.map((commit) => {
		const [hash, subject, unix_time] = commit.split("\n");
		plib.executeCommand(`git checkout --quiet ${hash}`);
		return {
			hash,
			subject,
			unix_time: Number(unix_time),
			code: countCode(),
			size: countSize(),
			words: countWords(),
		};
	});
}

function countCode() {
	return Array.from(
		new Set(
			plib
				.findFilesInDir("./", [/.py$|.sh$|.ts$|.gp$|.rs$/], [/archive\//])
				.flatMap((f) =>
					plib
						.readFile(f)
						.split("\n")
						.map((l) => l.trim())
						.filter((l) => l.length > 0),
				),
		),
	).length;
}

function countCovers() {
	return Array.from(
		new Set(
			plib
				.findFilesInDir(
					"./stories/",
					[/cover/],
					[/concept/, /.xcf$/, /upscaled/],
				)
				.map((c) => {
					const split_path = c.split(path.sep);
					return split_path
						.slice(0, split_path.indexOf("stories") + 2)
						.join(path.sep);
				}),
		),
	).length.toLocaleString("en-US");
}

function countMarkdownFiles(dir: string) {
	return plib.findFilesInDir(dir, [/\.md$/], []).length.toLocaleString("en-US");
}

function countFromFile(file: string, start: string) {
	return plib
		.readFile(path.join("./stories/", file))
		.split("\n")
		.filter((l) => l.startsWith(start))
		.length.toLocaleString("en-US");
}

function countSize() {
	return plib
		.findFilesInDir("./", [], [/archive\//, /\.git\//])
		.reduce((acc, file) => {
			const stats = fs.statSync(file);
			return acc + stats.size;
		}, 0);
}

function countDirs(folder: string) {
	return fs
		.readdirSync(folder)
		.filter((dir) => fs.lstatSync(path.join(folder, dir)).isDirectory())
		.length.toLocaleString("en-US");
}

function countWords() {
	return plib
		.findFilesInDir(
			"./",
			[/stories|flash-fiction/, /.md$/],
			[/archive\//, /meta.md$/, /ideas.md$/, /names.md$/],
		)
		.map((f) => {
			return plib
				.readFile(f)
				.replace(/<p align="center">\*\*\*<\/p>/g, "")
				.replace(/<center>\*\*\*<\/center>/g, "")
				.replace(/.*?\[.*?\]\(.*?\).*/g, "")
				.replace(/<!\-\-.*\-\->/g, "")
				.replace(/[#>\-*–|—]/g, "")
				.replace(/[\n\t]/g, " ")
				.replace(/ +/g, " ")
				.trim()
				.split(" ").length;
		})
		.reduce((a, b) => {
			return a + b;
		}, 0);
}

function formatSize(bytes: number) {
	const units = ["B", "KB", "MB", "GB", "TB"];
	let current = bytes;
	for (const unit of units) {
		if (current <= 1000) return `${current.toFixed(2)} ${unit}`;
		current /= 1000;
	}
	return `${bytes.toFixed(2)} ${units[0]}`;
}

function getChanges(pony_commits: PonyCommit[]) {
	if (pony_commits[0].code != 0)
		pony_commits[0].code_change = pony_commits[0].code;
	if (pony_commits[0].size != 0)
		pony_commits[0].size_change = pony_commits[0].size;
	if (pony_commits[0].words != 0)
		pony_commits[0].word_change = pony_commits[0].words;
	for (let i = 1; i < pony_commits.length; i++) {
		if (pony_commits[i].code - pony_commits[i - 1].code != 0)
			pony_commits[i].code_change =
				pony_commits[i].code - pony_commits[i - 1].code;
		if (pony_commits[i].size - pony_commits[i - 1].size != 0)
			pony_commits[i].size_change =
				pony_commits[i].size - pony_commits[i - 1].size;
		if (pony_commits[i].words - pony_commits[i - 1].words != 0)
			pony_commits[i].word_change =
				pony_commits[i].words - pony_commits[i - 1].words;
	}
	return pony_commits.reverse();
}
