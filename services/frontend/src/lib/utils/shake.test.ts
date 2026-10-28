import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { ShakeDetector } from './shake.svelte';

const requestPermission = vi.fn<() => Promise<'granted' | 'denied'>>();

let listeners: Array<{ type: string; listener: EventListenerOrEventListenerObject }> = [];

const setUserActivation = (isActive: boolean | undefined) => {
	Object.defineProperty(navigator, 'userActivation', {
		value: isActive === undefined ? undefined : { isActive },
		configurable: true
	});
};

const setupDetector = () => {
	const detector = new ShakeDetector();
	detector.isSupported = true;
	detector.init();

	return detector;
};

describe('ShakeDetector', () => {
	beforeEach(() => {
		requestPermission.mockReset();
		requestPermission.mockResolvedValue('granted');

		vi.stubGlobal(
			'DeviceMotionEvent',
			class {
				static requestPermission = requestPermission;
			}
		);
		setUserActivation(true);

		// Track the first-interaction listeners so they can be detached between tests.
		listeners = [];
		const addEventListener = window.addEventListener.bind(window);
		vi.spyOn(window, 'addEventListener').mockImplementation((type, listener, options) => {
			listeners.push({ type, listener });
			addEventListener(type, listener, options);
		});
	});

	afterEach(() => {
		for (const { type, listener } of listeners) {
			window.removeEventListener(type, listener);
		}

		vi.restoreAllMocks();
		vi.unstubAllGlobals();
		setUserActivation(undefined);
	});

	it('ignores touchstart, which carries no transient activation on iOS', () => {
		setupDetector();

		window.dispatchEvent(new Event('touchstart'));

		expect(requestPermission).not.toHaveBeenCalled();
	});

	it('requests permission on the first touchend', async () => {
		setupDetector();

		window.dispatchEvent(new Event('touchend'));

		await vi.waitFor(() => expect(requestPermission).toHaveBeenCalledTimes(1));
	});

	it('requests permission only once per page load', async () => {
		setupDetector();

		window.dispatchEvent(new Event('touchend'));
		window.dispatchEvent(new Event('touchend'));
		window.dispatchEvent(new Event('click'));

		await vi.waitFor(() => expect(requestPermission).toHaveBeenCalledTimes(1));

		expect(requestPermission).toHaveBeenCalledTimes(1);
	});

	it('keeps waiting when the gesture has no transient activation', async () => {
		setUserActivation(false);
		setupDetector();

		window.dispatchEvent(new Event('touchend'));
		expect(requestPermission).not.toHaveBeenCalled();

		setUserActivation(true);
		window.dispatchEvent(new Event('touchend'));

		await vi.waitFor(() => expect(requestPermission).toHaveBeenCalledTimes(1));
	});

	it('starts listening after permission is granted', async () => {
		const addEventListener = vi.spyOn(window, 'addEventListener');
		const detector = setupDetector();

		window.dispatchEvent(new Event('touchend'));

		await vi.waitFor(() => expect(detector.isEnabled).toBe(true));

		expect(addEventListener).toHaveBeenCalledWith('devicemotion', expect.any(Function));
	});

	it('warns without reporting an error when motion access is not allowed', async () => {
		requestPermission.mockRejectedValue(
			new DOMException(
				'Requesting device motion access requires a user gesture to prompt',
				'NotAllowedError'
			)
		);
		const consoleWarn = vi.spyOn(console, 'warn').mockImplementation(() => {});
		const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

		const detector = setupDetector();
		window.dispatchEvent(new Event('touchend'));

		await vi.waitFor(() => expect(consoleWarn).toHaveBeenCalled());

		expect(consoleError).not.toHaveBeenCalled();
		expect(detector.isEnabled).toBe(false);
	});

	it('reports unexpected failures as errors', async () => {
		requestPermission.mockRejectedValue(new Error('boom'));
		const consoleWarn = vi.spyOn(console, 'warn').mockImplementation(() => {});
		const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

		setupDetector();
		window.dispatchEvent(new Event('touchend'));

		await vi.waitFor(() => expect(consoleError).toHaveBeenCalled());

		expect(consoleWarn).not.toHaveBeenCalled();
	});
});
