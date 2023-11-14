import "@total-typescript/ts-reset";
import { execSync } from "child_process";

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
