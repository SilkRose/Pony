import "@total-typescript/ts-reset";
import { execSync } from "child_process";
import path from "path";
import fs from "fs";

export function rmDirs(dirs: string[]) {
	for (let dir of dirs) {
		if (fs.existsSync(dir)) {
			fs.rmSync(dir, { recursive: true, force: true });
		}
	}
}

export function mkDirs(dirs: string[]) {
	for (let dir of dirs) {
		fs.mkdirSync(dir, { recursive: true });
	}
}

export function findFilesInDir(
	dir: string,
	includes: RegExp[],
	excludes: RegExp[],
) {
	let files: string[] = [];
	if (!fs.existsSync(dir)) throw new Error("no dir " + dir);
	loop: for (const pathname of fs.readdirSync(dir)) {
		const name = path.join(dir, pathname);
		if (excludes.length > 0) {
			for (const exclude of excludes) {
				if (name.match(exclude)) continue loop;
			}
		}
		if (fs.lstatSync(name).isDirectory()) {
			files = files.concat(findFilesInDir(name, includes, excludes));
		} else {
			if (includes.length > 0) {
				for (const include of includes) {
					if (!name.match(include)) continue loop;
				}
			}
			files.push(name);
		}
	}
	return files;
}

export function writeFile(filename: string, data: string) {
	const filepath = path.resolve(path.join(process.cwd(), filename));
	fs.writeFileSync(filepath, data, { encoding: "utf-8" });
}

export function readFile(filename: string) {
	try {
		const filepath = path.resolve(filename);
		return fs.readFileSync(filepath, { encoding: "utf-8" });
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

export function checkInstalled(programs: string[]) {
	for (let program of programs) {
		try {
			execSync(`which "${program}"`);
		} catch (err) {
			throw new Error(`Exit: "${program}" is not installed.`);
		}
	}
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
