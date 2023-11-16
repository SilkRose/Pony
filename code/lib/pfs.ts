import "@total-typescript/ts-reset";
import path from "path";
import fs from "fs";

export function rmDir(dir: string) {
	if (fs.existsSync(dir)) {
		fs.rmSync(dir, { recursive: true, force: true });
	}
}

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
	excludes: RegExp[]
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

export function readFile(filename: string) {
	try {
		const filepath = path.resolve(filename);
		return fs.readFileSync(filepath, { encoding: "utf-8" });
	} catch (error) {
		throw new Error(`Failed to open file: ${filename}`);
	}
}
