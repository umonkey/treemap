type MapErrorEvent = {
	error?: unknown;
};

/**
 * Network failures that happen while a style or tile request is in flight are
 * not application errors. The browser cancels them when the user navigates
 * away, backgrounds the tab, or loses connectivity, and MapLibre surfaces them
 * as AJAX errors without an HTTP status. Reporting them only creates noise.
 */
const NON_ACTIONABLE_MESSAGES: RegExp[] = [
	/^Load failed/i, // WebKit/Safari
	/^Failed to fetch/i, // Chromium
	/^NetworkError when attempting to fetch resource/i, // Firefox
	/^Network request failed/i,
	/^The Internet connection appears to be offline/i,
	/^The network connection was lost/i,
	/^A server with the specified hostname could not be found/i,
	/^The request timed out/i
];

const getMessage = (error: unknown): string | undefined => {
	if (typeof error === 'string') {
		return error;
	}

	if (error instanceof Error) {
		return error.message;
	}

	if (typeof error === 'object' && error !== null && 'message' in error) {
		const message = (error as { message?: unknown }).message;

		if (typeof message === 'string') {
			return message;
		}
	}

	return undefined;
};

const getStatus = (error: unknown): number | undefined => {
	if (typeof error !== 'object' || error === null || !('status' in error)) {
		return undefined;
	}

	const status = (error as { status?: unknown }).status;

	return typeof status === 'number' ? status : undefined;
};

const isAbortError = (error: unknown): boolean => {
	return (
		typeof error === 'object' &&
		error !== null &&
		(error as { name?: unknown }).name === 'AbortError'
	);
};

/**
 * Tells whether a MapLibre error event is expected network noise that should not
 * be reported, as opposed to a genuinely broken style, source or layer.
 */
export const isNonActionableMapError = (event: MapErrorEvent): boolean => {
	const error = event.error;

	// MapLibre occasionally fires an error event without an error object.
	if (error === undefined || error === null) {
		return true;
	}

	if (isAbortError(error)) {
		return true;
	}

	// MapLibre's AJAXError reports status 0 when the request never completed:
	// aborted by a navigation or the network was unavailable.
	if (getStatus(error) === 0) {
		return true;
	}

	const message = getMessage(error);

	if (!message) {
		return false;
	}

	return NON_ACTIONABLE_MESSAGES.some((pattern) => pattern.test(message));
};
