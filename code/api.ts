#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { repository } from "./package.json";
import * as lib from "./lib.ts";

await mane();

const links = {
	pony: "https://pony.silkrose.dev/api/v1/pony.json",
	pony_commits: "https://pony.silkrose.dev/api/v1/pony-commits.json",
	hashes: "https://pony.silkrose.dev/shell-script-hashes",
};

async function mane() {
	lib.checkInstalled(["git"]);
	lib.rmDirs(["./dist", "./pony-temp"]);
	lib.mkDirs(["./dist/api/v1"]);
	lib.writeFile("./dist/.nojekyll", "");
	lib.writeFile("./dist/CNAME", "pony.silkrose.dev");
	const git_url = repository.url.slice(4);
	lib.executeCommand(`git clone --quiet ${git_url} pony-temp`);
	process.chdir("./pony-temp");
	console.log(process.cwd());
}
