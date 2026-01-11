import { sentrySvelteKit } from '@sentry/sveltekit';
/// <reference types="vitest" />
/// <reference types="vite/client" />

import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
	const env = loadEnv(mode, process.cwd(), '');
	const plugins = [];

	if (env.VITE_SENTRY_AUTH_TOKEN) {
		plugins.push(
			sentrySvelteKit({
				adapter: 'static',
				telemetry: false,
				sourceMapsUploadOptions: {
					org: 'trees-of-yerevan',
					project: 'treemap-v2',
					authToken: env.VITE_SENTRY_AUTH_TOKEN
				}
			})
		);
	}

	plugins.push(sveltekit());

	return {
		plugins,

		build: {
			sourcemap: true
		},

		test: {
			environment: 'jsdom',
			include: ['src/**/*.{test,spec}.{js,ts}'],
			setupFiles: ['./vitest.setup.ts']
		},

		resolve: process.env.VITEST
			? {
					conditions: ['browser']
				}
			: undefined
	};
});
