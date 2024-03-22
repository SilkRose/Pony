import z from "zod";
import puppeteer from "puppeteer";
import { promises as fs } from "fs";
import "@total-typescript/ts-reset";
import { execSync } from "child_process";

// schema for the cookie.
const cookie_schema = z.array(
	z.object({
		domain: z.string(),
		expires: z.number(),
		httpOnly: z.boolean(),
		name: z.string(),
		partitionKey: z.string().optional(),
		partitionKeyOpaque: z.boolean().optional(),
		path: z.string(),
		priority: z.enum(["Low", "Medium", "High"]).optional(),
		sameParty: z.boolean().optional(),
		sameSite: z.enum(["Strict", "Lax", "None"]).optional(),
		secure: z.boolean(),
		session: z.boolean(),
		size: z.number(),
		sourceScheme: z.enum(["Unset", "NonSecure", "Secure"]).optional(),
		value: z.string(),
	})
);

async function mane() {
	const story_id = process.argv[2];

	// checking the cookie expirery date.
	const cookies = cookie_schema.parse(
		JSON.parse(await fs.readFile(process.argv[4], "utf-8"))
	);
	const time = Date.now() / 1000;
	const expiry_date = cookies
		.filter((c) => c.name === "session_token")
		.map((c) => c.expires)[0];
	// check to see if the cookie expires within a month.
	if (time > expiry_date - 2592000) {
		execute_command(
			`notify-send "Expiring Cookie!" "The cookie provided for FIMFiction Bio will expire on ${new Date(
				expiry_date * 1000
			)}"`
		);
		console.warn(
			`The cookie provided for FIMFiction Bio will expire on ${new Date(
				expiry_date * 1000
			)}`
		);
	} else if (time > expiry_date) {
		execute_command(
			`notify-send -u critical "Expired Cookie!" "The cookie provided for FIMFiction Bio has expired on ${new Date(
				expiry_date * 1000
			)}"`
		);
		console.error(
			new Error(
				`The cookie provided for FIMFiction Bio has expired on ${new Date(
					expiry_date * 1000
				)}`
			)
		);
		process.exit(1);
	}

	// updating the cover.
	const browser = await puppeteer.launch({
		headless: "shell",
	});
	const page = await browser.newPage();
	await page.setCookie(...cookies);
	await page.goto("https://www.fimfiction.net/manage/stories/" + story_id, {
		waitUntil: "load",
	});
	const file_input = await page.$('input[name="story_image_file"]');
	await file_input!.uploadFile(process.argv[3]);
	await page.click("button.styled_button i.fa.fa-save");
	await page.waitForNetworkIdle();
	await browser.close();
}

mane();

function execute_command(command: string) {
	try {
		execSync(command);
	} catch (err) {
		throw new Error(`Failed to execute command: ${command}`);
	}
}
