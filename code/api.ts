#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { repository } from "./package.json" assert { type: "json" };
import * as pexec from "./lib/pexec.ts";
import * as pfs from "./lib/pfs.ts";
import * as pfetch from "./lib/pfetch.ts";
import path from "path";
import fs from "fs";

type Commit = {
	hash: string;
	subject: string;
	unix_time: number;
	stats: Stats;
};

type Stats = {
	covers: string;
	flash_fiction: string;
	ideas: string;
	names: string;
	stories: string;
	words: string;
};

const links = {
	pony: "https://pony.silkrose.dev/api/v1/pony.json",
	pony_commits: "https://pony.silkrose.dev/api/v1/pony-commits.json",
	template: "https://github.com/SilkRose/Pony/raw/{}/code/api.ts",
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
	let pony_commits: Commit[] | false = await pfetch.fetchJsonOrFalse(
		links.pony_commits
	);
	if (pony_commits)
		pony_commits = (await checkAPIFiles(pony_commits[0].hash))
			? pony_commits
			: false;
	pony_commits = false;
	const commits = getCommitData(git_log, pony_commits);
	console.log(commits.reverse());
}

async function checkAPIFiles(hash: string) {
	const latest = await pfetch.fetchOrFalse(
		links.template.replace("{}", "mane")
	);
	const latest_api = await pfetch.fetchOrFalse(
		links.template.replace("{}", hash)
	);
	if (!latest || !latest_api) return false;
	return latest === latest_api;
}

function getCommitData(git_log: string, pony_commits: Commit[] | false) {
	return git_log.split("\n\n").map((commit) => {
		const [hash, subject, unix_time] = commit.split("\n");
		return {
			hash,
			subject,
			unix_time: Number(unix_time),
			stats: getStats(hash, pony_commits),
		};
	});
}

function getStats(hash: string, pony_commits: Commit[] | false) {
	if (Array.isArray(pony_commits)) {
		const commit = pony_commits.find((c) => c.hash === hash) || false;
		if (commit) return commit.stats;
	}
	pexec.executeCommand(`git checkout --quiet ${hash}`);
	const stories_folder = getDirOrFalse("stories");
	const flash_fiction_folder = getDirOrFalse("flash-fiction");
	return {
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
		stories: stories_folder ? countDirs(stories_folder) : "0",
		words: "",
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

function countDirs(folder: string) {
	return fs
		.readdirSync(folder)
		.filter((dir) => fs.lstatSync(path.join(folder, dir)).isDirectory())
		.length.toLocaleString("en-US");
}
