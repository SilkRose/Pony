#!/usr/bin/env bun

import "@total-typescript/ts-reset";
import * as plib from "./lib.ts";
import path from "path";
import fs from "fs";

await mane();

async function mane() {
	const dir = path.resolve("../blog");
	for (const year of fs.readdirSync(dir)) {
		const year_dir = path.join(dir, year);
		for (const month of fs.readdirSync(year_dir)) {
			const month_dir = path.join(year_dir, month);
			plib
				.findFilesInDir(month_dir, [/.md$/], [])
				.map((f) => f.split("/").pop()!.split("-"))
				.sort((a, b) => {
					if (a[0] == b[0]) return Number(a[1]) - Number(b[1]);
					return Number(a[0]) - Number(b[0]);
				})
				.forEach((b) => {
					const old_name = path.join(month_dir, b.join("-"));
					const new_name = path.join(
						month_dir,
						b[0] + append_alpha(month_dir, b[0]) + ".md",
					);
					fs.renameSync(old_name, new_name);
				});
		}
	}
}

function append_alpha(dir: string, day: string) {
	const alphabet = "abcdefghijklmnopqrstuvwxyz";
	for (let c = 0; c < alphabet.length; c++) {
		const filename = path.join(dir, day + alphabet.slice(c, c + 1) + ".md");
		if (fs.existsSync(filename)) continue;
		return alphabet.slice(c, c + 1);
	}
	throw new Error();
}
