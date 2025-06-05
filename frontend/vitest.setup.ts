import { afterEach, beforeEach, vi } from 'vitest';

beforeEach(() => {
	const mockFetch = vi.fn();

	mockFetch.mockImplementation(async (req) => {
		console.warn(`[test] Failing an unexpected fetch call: ${req.url}`);
		throw new Error(`Unexpected fetch call: ${req.url}`);
	});

	vi.stubGlobal('fetch', mockFetch);
});

afterEach(() => {
	vi.unstubAllGlobals();
});

// This is necessary to make the tests work with the sound components.
// Vitest is using JSDOM which does not support the AudioContext API.
// When we import and UI component, the test would fail.
//
// The solution was found here:
//
// https://jestjs.io/docs/manual-mocks#mocking-methods-which-are-not-implemented-in-jsdom
Object.defineProperty(window, 'matchMedia', {
	writable: true,
	value: vi.fn().mockImplementation((query) => ({
		matches: false,
		media: query,
		onchange: null,
		addListener: vi.fn(), // deprecated
		removeListener: vi.fn(), // deprecated
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn()
	}))
});

// Mock this for the maplibre package.
// eslint-disable-next-line @typescript-eslint/no-unused-vars
window.URL.createObjectURL = (obj: Blob | MediaSource): string => 'blob:foobar';

// Mock ServiceWorkers, also required by maplibre.
global.Worker = vi.fn().mockImplementation((url: string) => {
	return {
		postMessage: vi.fn(),
		terminate: vi.fn(),
		onmessage: null,
		onerror: null,
		onmessageerror: null,
		onclose: null,
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		start: vi.fn(),
		stop: vi.fn(),
		url
	};
});

import 'vitest-canvas-mock';
