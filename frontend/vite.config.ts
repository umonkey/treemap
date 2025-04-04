import { sentrySvelteKit } from '@sentry/sveltekit';
/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

const getPlugins = () => {
	const plugins = [];

	if (process.env.VITE_SENTRY_AUTH_TOKEN) {
		plugins.push(
			sentrySvelteKit({
				sourceMapsUploadOptions: {
					org: 'trees-of-yerevan',
					project: 'treemap-v2',
					authToken: process.env.VITE_SENTRY_AUTH_TOKEN
				}
			})
		);
	}

	plugins.push(sveltekit());

	return plugins;
};

export default defineConfig({
	plugins: getPlugins(),

	test: {
		environment: 'jsdom',
		include: ['src/**/*.{test,spec}.{js,ts}']
	},

	resolve: process.env.VITEST
		? {
				conditions: ['browser']
			}
		: undefined
});
