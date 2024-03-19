import z from "zod";
import puppeteer from "puppeteer";
import { promises as fs } from "fs";
import "@total-typescript/ts-reset";

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

// schema for the recommendations file.
const recommendation_schema = z.array(
	z.object({
		"author-name": z.string(),
		"author-id": z.number(),
		"best-pony": z.string(),
		recommendations: z.array(
			z.object({
				"story-name": z.string(),
				"story-id": z.number(),
			})
		),
	})
);

// selectors the browser automation.
const edit_selector =
	'a.styled_button.styled_button_brown.edit-link[data-click="showEdit"]';
const text_field_selector = 'input[name="bio"]';
const save_selector = "button.styled_button i.fa.fa-save";

async function mane() {
	// contructing the bio.
	const recommendations = recommendation_schema.parse(
		JSON.parse(await fs.readFile(process.argv[2], "utf-8"))
	);
	const author =
		recommendations[Math.floor(Math.random() * recommendations.length)];
	const story =
		author.recommendations[
			Math.floor(Math.random() * author.recommendations.length)
		];
	const max_lenght = 200;
	const bio = `Go read [url=/story/${story["story-id"]}]${story["story-name"]}[/url], by [url=/user/${author["author-id"]}/]${author["author-name"]}[/url] | ${author["best-pony"]} is best pony! | Bio updates daily!`;

	// checking the cookie expirery date.
	const cookies = cookie_schema.parse(JSON.parse(await fs.readFile(process.argv[3], "utf-8")));
	const time = Date.now() / 1000;
	const expirery_date = cookies
		.filter((c) => c.name === "session_token")
		.map((c) => c.expires)[0];
	// check to see if the cookie expires within a month.
	if (time > expirery_date - 2592000) {
		console.warn("::warning ::Cookie expires in less than a month!");
	} else if (time > expirery_date) {
		console.error(new Error("::error ::Cookie has expired!"));
		process.exit(1);
	}

	// updating the bio.
	const browser = await puppeteer.launch({
		headless: "shell",
	});
	const page = await browser.newPage();
	await page.setCookie(...cookies);
	await page.goto("https://www.fimfiction.net/", {
		waitUntil: "load",
	});
	const user_profile_link = await page.evaluate(() => {
		const element = document.querySelector(".user_toolbar .fa-user");
		return element!.parentElement!.getAttribute("href");
	});
	await page.goto("https://www.fimfiction.net" + user_profile_link, {
		waitUntil: "load",
	});
	await page.click(edit_selector);
	await page.focus(text_field_selector);
	await page.keyboard.down("Control");
	await page.keyboard.press("KeyA");
	await page.keyboard.up("Control");
	await page.keyboard.press("Backspace");
	await page.type(text_field_selector, bio);
	await page.click(save_selector);
	await browser.close();
}

mane();
