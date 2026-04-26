import { chromium } from '@playwright/test';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const AUTH_FILE = path.join(__dirname, 'auth.json');
const OUTPUT_DIR = path.join(__dirname, '../static/assets/screenshots');

const ROUTES = [
	{ name: 'home', url: 'https://yerevan.treemaps.app/' },
	{ name: 'preview', url: 'https://yerevan.treemaps.app/tree/134346854707105792/preview' },
	{ name: 'details', url: 'https://yerevan.treemaps.app/tree/134346854707105792' },
	{ name: 'report', url: 'https://yerevan.treemaps.app/report?address=Komitas+avenue' },
	{
		name: 'add',
		url: 'https://yerevan.treemaps.app/add?lat=40.20652821799567&lng=44.514434201320114'
	}
];

const VIEWPORTS = [
	{
		name: 'narrow',
		width: 360,
		height: 800,
		scale: 3,
		mobile: true,
		touch: true,
		userAgent:
			'Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Mobile Safari/537.36'
	},
	{
		name: 'wide',
		width: 1920,
		height: 1080,
		scale: 1,
		mobile: false,
		touch: false,
		userAgent:
			'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36'
	}
];

async function generateScreenshots() {
	const isAuthSession = process.argv.includes('--auth');

	if (!fs.existsSync(OUTPUT_DIR)) {
		fs.mkdirSync(OUTPUT_DIR, { recursive: true });
	}

	const browser = await chromium.launch({
		headless: !isAuthSession,
		args: ['--disable-blink-features=AutomationControlled']
	});

	const contextOptions = {};
	if (fs.existsSync(AUTH_FILE) && !isAuthSession) {
		console.log('Loading existing auth state...');
		contextOptions.storageState = AUTH_FILE;
	}

	if (isAuthSession) {
		const context = await browser.newContext({
			...contextOptions,
			userAgent:
				'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36'
		});
		await context.addInitScript(() => {
			Object.defineProperty(navigator, 'webdriver', {
				get: () => undefined
			});
		});

		console.log('------------------------------------------------------------');
		console.log('AUTH MODE: Please log in to Google in the opened browser.');
		console.log('Once you are logged in and on the app home page, close the browser.');
		console.log('------------------------------------------------------------');
		const page = await context.newPage();
		await page.goto('https://yerevan.treemaps.app/');

		console.log('Waiting for you to close the browser window...');
		await page.waitForEvent('close', { timeout: 0 });

		await context.storageState({ path: AUTH_FILE });
		console.log(`Auth state saved to ${AUTH_FILE}`);
		await browser.close();
		return;
	}

	for (const viewport of VIEWPORTS) {
		console.log(`Capturing ${viewport.name} screenshots...`);

		const currentContextOptions = {
			...contextOptions,
			viewport: { width: viewport.width, height: viewport.height },
			deviceScaleFactor: viewport.scale,
			isMobile: viewport.mobile,
			hasTouch: viewport.touch,
			userAgent: viewport.userAgent
		};

		const context = await browser.newContext(currentContextOptions);

		// Stealth script to further hide playwright
		await context.addInitScript(() => {
			Object.defineProperty(navigator, 'webdriver', {
				get: () => undefined
			});
		});

		for (const route of ROUTES) {
			const page = await context.newPage();
			console.log(`  Navigating to ${route.url}...`);

			try {
				await page.goto(route.url, { waitUntil: 'networkidle', timeout: 60000 });

				// Wait for map if present
				const mapCanvas = await page.$('.maplibregl-canvas');
				if (mapCanvas) {
					console.log('    Waiting for map to stabilize...');
					await page.waitForTimeout(3000); // Give map some time to render tiles
				}

				const fileName = `screenshot-${viewport.name}-${route.name}.png`;
				const filePath = path.join(OUTPUT_DIR, fileName);

				await page.screenshot({ path: filePath, fullPage: false });
				console.log(`    Saved: ${fileName}`);
			} catch (error) {
				console.error(`    Failed to capture ${route.name}: ${error.message}`);
			} finally {
				await page.close();
			}
		}
		await context.close();
	}

	await browser.close();
	console.log('Done!');
}

generateScreenshots().catch(console.error);
