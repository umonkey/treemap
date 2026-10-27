import { sentrySvelteKit } from '@sentry/sveltekit';
/// <reference types="vitest" />
/// <reference types="vite/client" />

import { sveltekit } from '@sveltejs/kit/vite';
import type { PluginOption } from 'vite';
import { defineConfig } from 'vitest/config';

export default defineConfig(({ mode }) => {
	const plugins: PluginOption[] = [
		// Production builds must upload source maps; a map-less bundle is not acceptable.
		//
		// The check must not run while the config is merely being loaded: SvelteKit's
		// `svelte-kit sync` calls Vite's `resolveConfig({}, 'build', 'production')`,
		// which would trip a config-factory or `configResolved` check on every
		// `npm run check`. A `buildStart` hook only runs for an actual build.
		{
			name: 'require-sentry-upload-config',
			apply: 'build',
			buildStart() {
				if (mode !== 'production') {
					return;
				}

				if (!process.env.SENTRY_AUTH_TOKEN) {
					throw new Error(
						'SENTRY_AUTH_TOKEN is required for production builds (Sentry source-map upload).'
					);
				}
				if (!process.env.SENTRY_RELEASE) {
					throw new Error(
						'SENTRY_RELEASE is required for production builds (use the git commit SHA).'
					);
				}
			}
		}
	];

	const authToken = process.env.SENTRY_AUTH_TOKEN;
	const release = process.env.SENTRY_RELEASE;

	// The `require-sentry-upload-config` plugin above fails the build when either is missing.
	if (mode === 'production' && authToken && release) {
		plugins.push(
			sentrySvelteKit({
				adapter: 'other',
				telemetry: false,
				org: 'trees-of-yerevan',
				project: 'treemap-v2',
				authToken,
				release: { name: release },
				// Upload errors are only logged by default; make them fatal.
				errorHandler: (err) => {
					// `@sentry/sveltekit` swallows this rethrow inside its own
					// `closeBundle` and only logs "Failed to upload source maps!".
					// Mark the process as failed so the build still exits non-zero.
					process.exitCode = 1;
					throw err;
				},
				sourcemaps: {
					filesToDeleteAfterUpload: ['./build/**/*.map', './.svelte-kit/**/*.map']
				}
			})
		);
	}

	plugins.push(sveltekit());

	return {
		plugins,

		optimizeDeps: {
			esbuildOptions: {
				target: 'esnext'
			}
		},

		build: {
			sourcemap: 'hidden',
			target: 'esnext'
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
