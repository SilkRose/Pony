#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import * as plib from "./lib.ts";
import fs from "fs";

await mane();

async function mane() {
	const name = process.argv[2];
	const synopsis = process.argv[3] || "";
	const path = name.toLowerCase().replaceAll(" ", "-");
	const meta_md = plib
		.readFile("../markdown-templates/story-one-shot.md")
		.replace("# Title", `# ${name}`)
		.replace("## Synopsis:\n\n", `## Synopsis:\n${synopsis}\n`);
	const md = `# ${name}\n\n`;
	if (fs.existsSync(`../stories/${path}/`)) {
		throw new Error("Folder story already exists.");
	}
	plib.mkDirs([`../stories/${path}`]);
	plib.writeFile(`../stories/${path}/${path}-meta.md`, meta_md);
	plib.writeFile(`../stories/${path}/${path}.md`, md);
}
