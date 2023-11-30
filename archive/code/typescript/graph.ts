#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import * as pfs from "./lib/pfs.ts";
import { Commit } from "./api.ts";

await mane();

async function mane() {
	const jsonFilePath = "./dist/api/v1/pony-commits.json";
	const jsonData = (await pfs.readJsonFile(jsonFilePath)) as Commit[];
	const line_data = jsonData.map(
		(c) => `${c.unix_time} ${c.words} ${c.code} ${c.size}`,
	);
	const change_data = jsonData.reverse().map((c, i) => {
		if (i === 0) {
			return `${c.words} ${c.code} ${c.size}`;
		}
		const words = c.words - jsonData[i - 1].words;
		const code = c.code - jsonData[i - 1].code;
		const size = c.size - jsonData[i - 1].size;
		return `${words} ${code} ${size}`;
	});
	pfs.writeFile("./line.dat", line_data.join("\n"));
	pfs.writeFile("./box.dat", change_data.join("\n"));
}
