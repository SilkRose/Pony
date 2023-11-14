#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { repository } from "./package.json" assert { type: "json" };
import * as pexec from "./lib/pexec.ts";
import * as pfs from "./lib/pfs.ts";

await mane();

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
	hashes: "https://pony.silkrose.dev/shell-script-hashes",
};

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
		'git log mane --format="format:%H\n%s\n%ct"'
	);
	const commits = getCommitData(git_log);
	console.log(commits);
}

function getCommitData(git_log: string) {
	let commits: Commit[] = [];
	const log = git_log.split("\n");
	for (let i = 0; i < log.length; i += 3) {
		commits.push({
			hash: log[i],
			subject: log[i + 1],
			unix_time: Number(log[i + 2]),
			stats: {
				covers: "",
				flash_fiction: "",
				ideas: "",
				names: "",
				stories: "",
				words: "",
			},
		});
	}
	return commits;
}
