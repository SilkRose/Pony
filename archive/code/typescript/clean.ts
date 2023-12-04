#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import * as plib from "./lib.ts";

await mane();

async function mane() {
	plib.findFilesInDir("./", [/.md$/], [/README.md$/]).forEach((f) => {
		let file = plib
			.readFile(f)
			.replace(/[‘’\`´ʹ]/g, "'")
			.replace(/[“”‟″]/g, '"')
			.replaceAll("...", "…")
			.replaceAll(",*", "*,")
			.replaceAll(",_", "_,")
			.replaceAll("---", "—")
			.replaceAll("--", "–");
		plib.writeFile(f, file);
	});
}
