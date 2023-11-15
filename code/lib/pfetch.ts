import "@total-typescript/ts-reset";

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

export async function fetchOrFalse(url: string) {
	try {
		return await fetch(url).then((res) => res.text());
	} catch (err) {
		return false;
	}
}
