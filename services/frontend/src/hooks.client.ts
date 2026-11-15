import { config } from '$lib/env';
import { registerServiceWorker } from '$lib/utils/serviceWorker';
import { handleErrorWithSentry } from '@sentry/sveltekit';
import * as Sentry from '@sentry/sveltekit';

if (config.environment === 'production') {
	Sentry.init({
		dsn: 'https://ffcb68c9cff81ad4290341451f9b0623@o4507097921880064.ingest.de.sentry.io/4508460703481936',
		allowUrls: [/https:\/\/yerevan\.treemaps\.app\//],
		environment: config.environment,

		ignoreErrors: [
			// MapLibre GL does this when it fails to find an icon.
			'Please make sure you have added the image with map.addImage',
			// Native WKWebView bridge injected by the Facebook/Instagram iOS in-app
			// browser. Not our code; frames are attributed to the document URL.
			/window\.webkit\.messageHandlers/,
			// Headless Chromium bots (datacenter IPs, spoofed iOS/Android UAs, no GPU
			// process) throw this while MapLibre creates the map. Every recorded event in
			// FRONTEND-SA was such a bot; no real device has hit it.
			/webglcontextcreationerror|Failed to initialize WebGL/
		],

		// Enable API performance tracing.
		tracesSampleRate: 1.0,
		integrations: [
			Sentry.browserTracingIntegration(),
			Sentry.captureConsoleIntegration({ levels: ['error'] })
		],
		tracePropagationTargets: ['localhost', /^https:\/\/api\.treemaps\.app/],
		enableLogs: true
	});
}

// Registration failures are expected on some browsers and are handled as non-fatal.
registerServiceWorker();

// If you have a custom error handler, pass it to `handleErrorWithSentry`
export const handleError = handleErrorWithSentry();
