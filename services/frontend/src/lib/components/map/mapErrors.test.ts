import { describe, expect, it } from 'vitest';
import { isNonActionableMapError } from './mapErrors';

describe('isNonActionableMapError', () => {
	it('ignores an error event without an error object', () => {
		expect(isNonActionableMapError({})).toBe(true);
	});

	it('ignores AbortError, whether a DOMException or a plain Error', () => {
		expect(isNonActionableMapError({ error: new DOMException('Aborted', 'AbortError') })).toBe(
			true
		);

		const error = new Error('The operation was aborted.');
		error.name = 'AbortError';

		expect(isNonActionableMapError({ error })).toBe(true);
	});

	it('ignores AJAX errors that never completed (status 0)', () => {
		// This is the shape reported as TREEMAP-V2-QC on Mobile Safari.
		const error = Object.assign(new Error('Load failed (api.maptiler.com)'), {
			name: 'Error',
			status: 0,
			statusText: 'Load failed (api.maptiler.com)'
		});

		expect(isNonActionableMapError({ error })).toBe(true);
	});

	it.each([
		'Load failed',
		'Failed to fetch',
		'NetworkError when attempting to fetch resource.',
		'Network request failed',
		'The Internet connection appears to be offline.',
		'The network connection was lost.'
	])('ignores the browser network failure %j', (message) => {
		expect(isNonActionableMapError({ error: new TypeError(message) })).toBe(true);
	});

	it('ignores network failures reported as a string', () => {
		expect(isNonActionableMapError({ error: 'Load failed' })).toBe(true);
	});

	it.each([403, 404, 429, 500])('reports AJAX errors with HTTP status %i', (status) => {
		const error = Object.assign(new Error(`Forbidden (api.maptiler.com) (${status})`), {
			status
		});

		expect(isNonActionableMapError({ error })).toBe(false);
	});

	it('reports genuine style and source errors', () => {
		expect(isNonActionableMapError({ error: new Error('Style is not done loading') })).toBe(false);
		expect(isNonActionableMapError({ error: new Error('Failed to load sprite image') })).toBe(
			false
		);
	});
});
