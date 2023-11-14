#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import { rmDir, mkDirs, checkInstalled, writeFile } from "./lib.ts";

await mane();

async function mane() {
	checkInstalled(["git"]);
	rmDir("./dist");
	mkDirs(["./dist/api/v1"]);
	writeFile("./dist/.nojekyll", "");
	writeFile("./dist/CNAME", "pony.silkrose.dev");
}
