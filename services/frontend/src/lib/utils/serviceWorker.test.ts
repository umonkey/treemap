import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const env = vi.hoisted(() => ({ browser: true, dev: false }));

vi.mock('$app/environment', () => env);

const register = vi.fn();

function mockRegistration(value: ServiceWorkerRegistration) {
	register.mockResolvedValue(value);
}

describe('registerServiceWorker', () => {
	beforeEach(() => {
		vi.resetModules();
		vi.restoreAllMocks();
		register.mockReset();
		env.browser = true;
		env.dev = false;

		vi.stubGlobal('navigator', { serviceWorker: { register } });
		vi.spyOn(document, 'readyState', 'get').mockReturnValue('complete');
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('registers the service worker and resolves with the registration', async () => {
		const registration = { scope: '/' } as unknown as ServiceWorkerRegistration;
		mockRegistration(registration);

		const { registerServiceWorker } = await import('./serviceWorker');

		await expect(registerServiceWorker()).resolves.toBe(registration);
		expect(register).toHaveBeenCalledOnce();
		expect(register).toHaveBeenCalledWith('/service-worker.js');
	});

	it('resolves undefined instead of rejecting when registration fails', async () => {
		const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
		register.mockRejectedValue(new Error('Blocked by the browser'));

		const { registerServiceWorker } = await import('./serviceWorker');

		await expect(registerServiceWorker()).resolves.toBeUndefined();
		expect(warn).toHaveBeenCalledOnce();
	});

	it('reuses a single registration attempt', async () => {
		mockRegistration({} as ServiceWorkerRegistration);

		const { registerServiceWorker } = await import('./serviceWorker');

		await Promise.all([registerServiceWorker(), registerServiceWorker()]);
		expect(register).toHaveBeenCalledOnce();
	});

	it('waits for the load event before registering', async () => {
		vi.spyOn(document, 'readyState', 'get').mockReturnValue('loading');
		mockRegistration({} as ServiceWorkerRegistration);

		const { registerServiceWorker } = await import('./serviceWorker');

		const result = registerServiceWorker();
		expect(register).not.toHaveBeenCalled();

		window.dispatchEvent(new Event('load'));
		await result;
		expect(register).toHaveBeenCalledOnce();
	});

	it('does nothing in dev', async () => {
		env.dev = true;

		const { registerServiceWorker } = await import('./serviceWorker');

		await expect(registerServiceWorker()).resolves.toBeUndefined();
		expect(register).not.toHaveBeenCalled();
	});

	it('does nothing when the browser has no service worker support', async () => {
		vi.stubGlobal('navigator', {});

		const { registerServiceWorker } = await import('./serviceWorker');

		await expect(registerServiceWorker()).resolves.toBeUndefined();
		expect(register).not.toHaveBeenCalled();
	});
});
