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
	const filepath = path.resolve(path.join(process.cwd(), filename));
	fs.writeFileSync(filepath, data, { encoding: "utf-8" });
}
