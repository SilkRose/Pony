#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import * as plib from "./lib.ts";
import path from "path";

await mane();

async function mane() {
	plib.checkInstalled(["fimd"]);
	plib.rmDirs(["./publish"]);
	plib.mkDirs(["./publish"]);
	plib.findFilesInDir(
		"../",
		[/.md$/, /stories|flash-fiction/],
		[/archive/, /ideas.md$/, /names.md$/]
	).forEach((md) => {
		const bbcode = md.replace("../", "./publish/").replace(".md", ".txt");
		plib.mkDirs([path.dirname(bbcode)]);
		plib.executeCommand(`fimd ${md} ${bbcode}`);
		console.log("Converted: " + md);
	});
}
