#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { repository } from "./package.json" assert { type: "json" };
import * as pexec from "./lib/pexec.ts";
import * as pfmt from "./lib/pfmt.ts";
import * as pfs from "./lib/pfs.ts";
import path from "path";
import fs from "fs";

type Commit = {
	hash: string;
	subject: string;
	unix_time: number;
	stats: Stats;
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

await mane();

async function mane() {
	pexec.checkInstalled(["git"]);
	pfs.rmDirs(["./dist", "./pony-temp"]);
	pfs.mkDirs(["./dist/api/v1"]);
	pfs.writeFile("./dist/.nojekyll", "");
	pfs.writeFile("./dist/CNAME", "pony.silkrose.dev");
	const git_url = repository.url.slice(4);
	pexec.executeCommand(`git clone --quiet ${git_url} pony-temp`);
	process.chdir("./pony-temp");
	const git_log = pexec.executeCommandReturn(
		'git log mane --format="format:%H\n%s\n%ct\n"'
	);
	const commits: Commit[] = getCommitData(git_log);
	const pony_string = pfmt.jsonFmt(JSON.stringify(commits[0].stats));
	const pony_commits_string = pfmt.jsonFmt(JSON.stringify(commits));
	pfs.writeFile("../dist/api/v1/pony.json", pony_string + "\n");
	pfs.writeFile(
		"../dist/api/v1/pony-commits.json",
		pony_commits_string + "\n"
	);
}

function getCommitData(git_log: string) {
	return git_log.split("\n\n").map((commit) => {
		const [hash, subject, unix_time] = commit.split("\n");
		return {
			hash,
			subject,
			unix_time: Number(unix_time),
			stats: getStats(hash),
		};
	});
}

function getStats(hash: string) {
	pexec.executeCommand(`git checkout --quiet ${hash}`);
	const stories_folder = getDirOrFalse("stories");
	const flash_fiction_folder = getDirOrFalse("flash-fiction");
	return {
		code: countCode(),
		covers: stories_folder ? countCovers(stories_folder) : "0",
		flash_fiction: flash_fiction_folder
			? countFlashFiction(flash_fiction_folder)
			: "0",
		ideas: stories_folder
			? countFromFile(stories_folder, "ideas.md", "## ")
			: "0",
		names: stories_folder
			? countFromFile(stories_folder, "names.md", "- ")
			: "0",
		size: countSize(),
		stories: stories_folder ? countDirs(stories_folder) : "0",
		words: countWords(stories_folder, flash_fiction_folder),
	};
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
			pfs
				.findFilesInDir(
					"./",
					[/\.py$|\.sh$|\.ts$|\.rs$/],
					[/archive\//]
				)
				.flatMap((f) =>
					pfs
						.readFile(f)
						.split("\n")
						.map((l) => l.trim())
						.filter((l) => l.length > 0)
				)
		)
	).length.toLocaleString("en-US");
}

function countCovers(stories_folder: string) {
	return Array.from(
		new Set(
			pfs
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
	).length.toLocaleString("en-US");
}

function countFlashFiction(flash_fiction_folder: string) {
	return pfs
		.findFilesInDir(flash_fiction_folder, [/\.md$/], [])
		.length.toLocaleString("en-US");
}

function countFromFile(folder: string, file: string, start: string) {
	if (fs.existsSync(path.join(folder, file))) {
		return pfs
			.readFile(path.join(folder, file))
			.split("\n")
			.filter((l) => l.startsWith(start))
			.length.toLocaleString("en-US");
	} else {
		return "0";
	}
}

function countSize() {
	return pfs
		.findFilesInDir("./", [], [/archive\//, /\.git\//])
		.map((f) => fs.statSync(f).size)
		.reduce((a, b) => a + b)
		.toLocaleString("en-US");
}

function countDirs(folder: string) {
	return fs
		.readdirSync(folder)
		.filter((dir) => fs.lstatSync(path.join(folder, dir)).isDirectory())
		.length.toLocaleString("en-US");
}

function countWords(
	stories_folder: string | false,
	flash_fiction_folder: string | false
) {
	if (!stories_folder && !flash_fiction_folder) return "0";
	if (!stories_folder) return "0";
	const story_files = pfs.findFilesInDir(
		stories_folder,
		[/.md$/],
		[/meta.md$/, /ideas.md$/, /names.md$/]
	);
	const flash_fiction_files = !flash_fiction_folder
		? []
		: pfs.findFilesInDir(flash_fiction_folder, [/.md$/], []);
	return story_files
		.concat(flash_fiction_files)
		.map((f) => {
			return pfs
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
		.reduce((a, b) => a + b)
		.toLocaleString("en-US");
}
