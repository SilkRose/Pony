import "@total-typescript/ts-reset";
import path from "path";
import fs from "fs";

export async function fetchJsonData(url: string) {
	return await fetch(url).then((res) => res.json());
}

export async function fetchJsonOrFalse(url: string) {
	try {
		return await fetch(url).then((res) => res.json());
	} catch (err) {
		return false;
	}
}

export async function readJsonFile(filename: string) {
	try {
		const filepath = path.resolve(path.join(process.cwd(), filename));
		return await JSON.parse(
			fs.readFileSync(filepath, { encoding: "utf-8" })
		);
	} catch (error) {
		throw new Error(`Failed to open file: ${filename}`);
	}
}

export function jsonFmt(json: string) {
	return JSON.stringify(JSON.parse(json), null, "\t");
}

export function jsonMinify(json: string) {
	return JSON.stringify(JSON.parse(json));
}
