import "@total-typescript/ts-reset";
import path from "path";
import fs from "fs";
import { execSync } from "child_process";

export function rmDir(dir: string) {
	if (fs.existsSync(dir)) {
		fs.rmSync(dir, { recursive: true, force: true });
	}
}

export function mkDirs(dirs: string[]) {
	for (let dir of dirs) {
		fs.mkdir(dir, { recursive: true }, (err) => {
			if (err) throw err;
		});
	}
}

export function checkInstalled(programs: string[]) {
	for (let program of programs) {
		try {
			execSync(`which "${program}"`);
		} catch (err) {
			throw new Error(`Exit: "${program}" is not installed.`);
		}
	}
}

export function findFilesInDir(startPath: string, extension: string) {
	let results: string[] = [];
	if (!fs.existsSync(startPath)) {
		console.log("no dir ", startPath);
		throw Error;
	}
	let files = fs.readdirSync(startPath);
	for (let i = 0; i < files.length; i++) {
		let filename = path.join(startPath, files[i]);
		let stat = fs.lstatSync(filename);
		if (stat.isDirectory()) {
			results = results.concat(findFilesInDir(filename, extension));
		} else if (filename.endsWith(extension)) {
			results.push(filename);
		}
	}
	return results;
}

export function writeFile(filename: string, data: string) {
	fs.writeFileSync(filename, data);
}

export async function fetchJsonData(url: string) {
	return await fetch(url).then((res) => res.json());
}

export function jsonFmt(json: string) {
	return JSON.stringify(JSON.parse(json), null, "\t");
}

export function jsonMinify(json: string) {
	return JSON.stringify(JSON.parse(json));
}

export function executeCommand(command: string) {
	try {
		execSync(command);
	} catch (err) {
		throw new Error(`Failed to execute command: ${command}`);
	}
}

export function executeCommandReturn(command: string) {
	try {
		return execSync(command).toString();
	} catch (err) {
		throw new Error(`Failed to execute command: ${command}`);
	}
}
