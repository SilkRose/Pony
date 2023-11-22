#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { repository } from "./package.json" assert { type: "json" };
import * as plib from "./lib.ts";
import path from "path";
import fs from "fs";

type Commit = {
	hash: string;
	subject: string;
	unix_time: number;
	code: number;
	covers: number;
	flash_fiction: number;
	ideas: number;
	names: number;
	size: number;
	stories: number;
	words: number;
};

type Stats = {
	code: string;
	covers: string;
	flash_fiction: string;
	ideas: string;
	names: string;
	size: string;
	stories: string;
	words: string;
};

type Change = Commit;

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
	const git_log = plib.executeCommandReturn(
		'git log mane --format="format:%H\n%s\n%ct\n"'
	);
	const commits: Commit[] = getCommitData(git_log);
	const stats: Stats = getLatestStats(commits[0]);
	const pony_string = plib.jsonFmt(JSON.stringify(stats));
	const pony_commits_string = plib.jsonFmt(JSON.stringify(commits));
	plib.writeFile("../dist/api/v1/pony.json", pony_string + "\n");
	plib.writeFile(
		"../dist/api/v1/pony-commits.json",
		pony_commits_string + "\n"
	);
	const changes = getChanges(commits);
	const changes_string = plib.jsonFmt(JSON.stringify(changes));
	plib.writeFile("../dist/api/v1/pony-changes.json", changes_string + "\n");
}

function getCommitData(git_log: string) {
	return git_log.split("\n\n").map((commit) => {
		const [hash, subject, unix_time] = commit.split("\n");
		plib.executeCommand(`git checkout --quiet ${hash}`);
		const stories_folder = getDirOrFalse("stories");
		const flash_fiction_folder = getDirOrFalse("flash-fiction");
		return {
			hash,
			subject,
			unix_time: Number(unix_time),
			code: countCode(),
			covers: countCovers(stories_folder),
			flash_fiction: countFlashFiction(flash_fiction_folder),
			ideas: countFromFile(stories_folder, "ideas.md", "## "),
			names: countFromFile(stories_folder, "names.md", "- "),
			size: countSize(),
			stories: countDirs(stories_folder),
			words: countWords(stories_folder, flash_fiction_folder),
		};
	});
}

function getDirOrFalse(dir: string) {
	if (fs.existsSync(path.resolve("./" + dir))) {
		return path.resolve("./" + dir);
	} else if (fs.existsSync(path.resolve("./src/" + dir))) {
		return path.resolve("./src/" + dir);
	} else {
		return false;
	}
}

function countCode() {
	return Array.from(
		new Set(
			plib
				.findFilesInDir(
					"./",
					[/\.py$|\.sh$|\.ts$|\.gp$|\.rs$/],
					[/archive\//]
				)
				.flatMap((f) =>
					plib
						.readFile(f)
						.split("\n")
						.map((l) => l.trim())
						.filter((l) => l.length > 0)
				)
		)
	).length;
}

function countCovers(stories_folder: string | false) {
	if (!stories_folder) return 0;
	return Array.from(
		new Set(
			plib
				.findFilesInDir(
					stories_folder,
					[/cover/],
					[/concept/, /\.xcf$/, /upscaled/]
				)
				.map((c) => {
					const split_path = c.split(path.sep);
					return split_path
						.slice(0, split_path.indexOf("stories") + 2)
						.join(path.sep);
				})
		)
	).length;
}

function countFlashFiction(flash_fiction_folder: string | false) {
	if (!flash_fiction_folder) return 0;
	return plib.findFilesInDir(flash_fiction_folder, [/\.md$/], []).length;
}

function countFromFile(folder: string | false, file: string, start: string) {
	if (!folder) return 0;
	if (fs.existsSync(path.join(folder, file))) {
		return plib
			.readFile(path.join(folder, file))
			.split("\n")
			.filter((l) => l.startsWith(start)).length;
	} else {
		return 0;
	}
}

function countSize() {
	return plib
		.findFilesInDir("./", [], [/archive\//, /\.git\//])
		.map((f) => fs.statSync(f).size)
		.reduce((a, b) => a + b);
}

function countDirs(folder: string | false) {
	if (!folder) return 0;
	return fs
		.readdirSync(folder)
		.filter((dir) => fs.lstatSync(path.join(folder, dir)).isDirectory())
		.length;
}

function countWords(
	stories_folder: string | false,
	flash_fiction_folder: string | false
) {
	if (!stories_folder && !flash_fiction_folder) return 0;
	if (!stories_folder) return 0;
	const story_files = plib.findFilesInDir(
		stories_folder,
		[/.md$/],
		[/meta.md$/, /ideas.md$/, /names.md$/]
	);
	const flash_fiction_files = !flash_fiction_folder
		? []
		: plib.findFilesInDir(flash_fiction_folder, [/.md$/], []);
	return story_files
		.concat(flash_fiction_files)
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
		.reduce((a, b) => a + b);
}

function getLatestStats(latest: Commit) {
	return {
		code: latest.code.toLocaleString("en-US"),
		covers: latest.covers.toLocaleString("en-US"),
		flash_fiction: latest.flash_fiction.toLocaleString("en-US"),
		ideas: latest.ideas.toLocaleString("en-US"),
		names: latest.names.toLocaleString("en-US"),
		size: formatSize(latest.size),
		stories: latest.stories.toLocaleString("en-US"),
		words: latest.words.toLocaleString("en-US"),
	};
}

function formatSize(bytes: number) {
	const units = ["B", "KB", "MB", "GB", "TB"];
	let current = bytes;
	for (const unit of units) {
		if (current <= 1000) {
			return `${current.toFixed(1)} ${unit}`;
		}
		current /= 1000;
	}
	return `${bytes.toFixed(1)} ${units[0]}`;
}

function getChanges(pony_commits: Commit[]) {
	const change_data: Change[] = [];
	change_data.push(pony_commits[pony_commits.length - 1]);
	for (let i = pony_commits.length - 2; i >= 0; i--) {
		const c = pony_commits[i];
		const base = pony_commits[i + 1];
		change_data.push({
			hash: c.hash,
			subject: c.subject,
			unix_time: c.unix_time,
			code: c.code - base.code,
			covers: c.covers - base.covers,
			flash_fiction: c.flash_fiction - base.flash_fiction,
			ideas: c.ideas - base.ideas,
			names: c.names - base.names,
			size: c.size - base.size,
			stories: c.stories - base.stories,
			words: c.words - base.words,
		});
	}
	return change_data.reverse();
}
