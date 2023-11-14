#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { repository } from "./package.json" assert { type: "json" };
import * as pexec from "./lib/pexec.ts";
import * as pfs from "./lib/pfs.ts";

await mane();

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
	console.log(process.cwd());
}
