import z from "zod";
import puppeteer from "puppeteer";
import readlineSync from "readline-sync";
import { promises as fsPromises } from "fs";
import "@total-typescript/ts-reset";

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

const name_selector = 'input[name="username"]';
const password_selector = 'input[name="password"]';
const login_selector = "button.styled_button i.fa.fa-sign-in";

async function mane() {
	const browser = await puppeteer.launch({
		headless: "shell",
	});
	const page = await browser.newPage();
	await page.goto("https://www.fimfiction.net/", {
		waitUntil: "load",
	});
	await page.type(name_selector, input_username());
	await page.type(password_selector, input_password());
	await page.click(login_selector);
	await page.waitForNavigation();
	const cookies = cookie_schema.parse(await page.cookies());
	await fsPromises.writeFile("cookies.json", JSON.stringify(cookies));
	await browser.close();
}

function input_username() {
	return readlineSync.question("Enter your username or email: ");
}

function input_password() {
	return readlineSync.question("Enter your password (hidden input): ", {
		hideEchoBack: true,
	});
}

mane();
