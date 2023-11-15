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
		fs.mkdir(dir, { recursive: true }, (err) => {
			if (err) throw err;
		});
	}
}

export function findFilesInDir(
	dir: string,
	exts: string[],
	excludes: RegExp[]
) {
	let files: string[] = [];
	if (!fs.existsSync(dir)) throw new Error("no dir " + dir);
	loop: for (const pathname of fs.readdirSync(dir)) {
		const name = path.join(dir, pathname);
		if (excludes.length > 0) {
			for (const exFile of excludes) {
				if (name.match(exFile)) continue loop;
			}
		}
		if (fs.lstatSync(name).isDirectory()) {
			files = files.concat(findFilesInDir(name, exts, excludes));
		} else if (exts.length > 0) {
			for (const ext of exts) {
				if (name.endsWith(ext)) {
					files.push(name);
					break;
				}
			}
		} else {
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
