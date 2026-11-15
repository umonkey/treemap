import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const ILLEGAL_INVOCATION = "Failed to execute 'query' on 'Permissions': Illegal invocation";

type PermissionQuery = (
	descriptor: PermissionDescriptor
) => Promise<Pick<PermissionStatus, 'state'> | null>;

interface BrowserStub {
	permissions?: { query: PermissionQuery } | undefined;
	geolocation?: {
		watchPosition: (success: unknown, error: unknown, options: unknown) => number;
		clearWatch: (id: number) => void;
	};
}

const watchPosition = vi.fn(() => 1);
const clearWatch = vi.fn();

const geolocation = { watchPosition, clearWatch };

const stubBrowser = ({ permissions, geolocation: geolocationStub }: BrowserStub) => {
	Object.defineProperty(navigator, 'permissions', {
		configurable: true,
		value: permissions
	});
	Object.defineProperty(navigator, 'geolocation', {
		configurable: true,
		value: geolocationStub
	});
};

// The tracker is a module-level singleton, so every test needs a fresh module.
const loadTracker = async () => {
	vi.resetModules();
	const { locationTracker } = await import('./LocationTracker.svelte.ts');

	return locationTracker;
};

describe('LocationTracker permission pre-check', () => {
	let unmount: (() => void) | undefined;

	beforeEach(() => {
		watchPosition.mockReset();
		watchPosition.mockImplementation(() => 1);
		clearWatch.mockReset();
	});

	afterEach(() => {
		// The cleanup detaches the focus/visibilitychange listeners and stops the watch.
		unmount?.();
		unmount = undefined;

		// These stubs are own properties on navigator, so they survive `vi.unstubAllGlobals`.
		Reflect.deleteProperty(navigator, 'permissions');
		Reflect.deleteProperty(navigator, 'geolocation');

		vi.restoreAllMocks();
	});

	it('starts tracking when geolocation permission is granted', async () => {
		const query = vi.fn<PermissionQuery>().mockResolvedValue({ state: 'granted' });
		stubBrowser({ permissions: { query }, geolocation });

		const tracker = await loadTracker();
		unmount = tracker.onMount();

		await vi.waitFor(() => expect(watchPosition).toHaveBeenCalledOnce());
	});

	it('does not start tracking when permission is only prompted', async () => {
		const debug = vi.spyOn(console, 'debug').mockImplementation(() => {});
		const query = vi.fn<PermissionQuery>().mockResolvedValue({ state: 'prompt' });
		stubBrowser({ permissions: { query }, geolocation });

		const tracker = await loadTracker();
		unmount = tracker.onMount();

		await vi.waitFor(() =>
			expect(debug).toHaveBeenCalledWith('[GEO] Permissions are prompt, not starting.')
		);
		expect(watchPosition).not.toHaveBeenCalled();
	});

	it('treats a rejecting permission query as unknown', async () => {
		const debug = vi.spyOn(console, 'debug').mockImplementation(() => {});
		const query = vi.fn<PermissionQuery>().mockRejectedValue(new TypeError(ILLEGAL_INVOCATION));
		stubBrowser({ permissions: { query }, geolocation });

		const tracker = await loadTracker();

		expect(() => {
			unmount = tracker.onMount();
		}).not.toThrow();

		await vi.waitFor(() =>
			expect(debug).toHaveBeenCalledWith(
				'[GEO] Could not query permissions:',
				expect.any(TypeError)
			)
		);
		expect(watchPosition).not.toHaveBeenCalled();
	});

	it('treats a synchronously throwing permission query as unknown', async () => {
		const debug = vi.spyOn(console, 'debug').mockImplementation(() => {});
		const query = vi.fn<PermissionQuery>(() => {
			throw new TypeError(ILLEGAL_INVOCATION);
		});
		stubBrowser({ permissions: { query }, geolocation });

		const tracker = await loadTracker();

		expect(() => {
			unmount = tracker.onMount();
		}).not.toThrow();

		await vi.waitFor(() =>
			expect(debug).toHaveBeenCalledWith(
				'[GEO] Could not query permissions:',
				expect.any(TypeError)
			)
		);
		expect(watchPosition).not.toHaveBeenCalled();
	});

	it('does not start tracking when the Permissions API is missing', async () => {
		const debug = vi.spyOn(console, 'debug').mockImplementation(() => {});
		stubBrowser({ permissions: undefined, geolocation });

		const tracker = await loadTracker();
		unmount = tracker.onMount();

		await vi.waitFor(() =>
			expect(debug).toHaveBeenCalledWith('[GEO] Could not query permissions.')
		);
		expect(watchPosition).not.toHaveBeenCalled();
	});

	it('treats a null permission status as unknown', async () => {
		const debug = vi.spyOn(console, 'debug').mockImplementation(() => {});
		const query = vi.fn<PermissionQuery>().mockResolvedValue(null);
		stubBrowser({ permissions: { query }, geolocation });

		const tracker = await loadTracker();
		unmount = tracker.onMount();

		await vi.waitFor(() =>
			expect(debug).toHaveBeenCalledWith('[GEO] Permissions are unknown, not starting.')
		);
		expect(watchPosition).not.toHaveBeenCalled();
	});
});
