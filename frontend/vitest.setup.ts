import { afterEach, beforeEach, vi } from "vitest";

beforeEach(() => {
	const mockFetch = vi.fn();

	mockFetch.mockImplementation(async (req, options) => {
		console.warn(`[test] Failing an unexpected fetch call: ${req.url}`);
		throw new Error(`Unexpected fetch call: ${req.url}`);
	});

	vi.stubGlobal("fetch", mockFetch);

	console.debug("[test] Fetch call stubbed.");
});

afterEach(() => {
	vi.unstubAllGlobals();
});
