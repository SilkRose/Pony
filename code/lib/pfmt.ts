import "@total-typescript/ts-reset";

export function jsonFmt(json: string) {
	return JSON.stringify(JSON.parse(json), null, "\t");
}

export function jsonMinify(json: string) {
	return JSON.stringify(JSON.parse(json));
}
