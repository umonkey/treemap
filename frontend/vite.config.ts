import { sentrySvelteKit } from "@sentry/sveltekit";
/// <reference types="vitest" />
/// <reference types="vite/client" />

import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";

const getPlugins = () => {
	const plugins = [];

	if (process.env.VITE_SENTRY_AUTH_TOKEN) {
		plugins.push(
			sentrySvelteKit({
				sourceMapsUploadOptions: {
					org: "trees-of-yerevan",
					project: "treemap-v2",
					authToken: process.env.VITE_SENTRY_AUTH_TOKEN,
				},
			}),
		);
	}

	plugins.push(sveltekit());

	return plugins;
};

export default defineConfig({
	plugins: getPlugins(),

	test: {
		environment: "jsdom",
		include: ["src/**/*.{test,spec}.{js,ts}"],
		setupFiles: ["./vitest.setup.ts"],
	},

	resolve: process.env.VITEST
		? {
				conditions: ["browser"],
			}
		: undefined,
});
