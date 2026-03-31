import { config } from '$lib/env';
import { handleErrorWithSentry } from '@sentry/sveltekit';
import * as Sentry from '@sentry/sveltekit';

if (config.environment === 'production') {
	Sentry.init({
		dsn: 'https://ffcb68c9cff81ad4290341451f9b0623@o4507097921880064.ingest.de.sentry.io/4508460703481936',
		allowUrls: [/https:\/\/yerevan\.treemaps\.app\//],
		environment: config.environment,

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

// If you have a custom error handler, pass it to `handleErrorWithSentry`
export const handleError = handleErrorWithSentry();
