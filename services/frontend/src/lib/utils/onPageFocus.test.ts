import { describe, expect, it, vi } from 'vitest';
import { onPageFocus } from './onPageFocus';

describe('onPageFocus', () => {
	it('registers focus and visibilitychange listeners', () => {
		const addEventListener = vi.spyOn(window, 'addEventListener');
		const documentAddEventListener = vi.spyOn(document, 'addEventListener');

		onPageFocus(() => {});

		expect(addEventListener).toHaveBeenCalledWith('focus', expect.any(Function));
		expect(documentAddEventListener).toHaveBeenCalledWith('visibilitychange', expect.any(Function));

		addEventListener.mockRestore();
		documentAddEventListener.mockRestore();
	});

	it('calls the callback on focus when visible', () => {
		const cb = vi.fn();
		onPageFocus(cb);

		window.dispatchEvent(new Event('focus'));

		expect(cb).toHaveBeenCalledTimes(1);
	});

	it('skips the callback when hidden', () => {
		const cb = vi.fn();
		const spy = vi.spyOn(document, 'visibilityState', 'get').mockReturnValue('hidden');
		onPageFocus(cb);

		window.dispatchEvent(new Event('focus'));

		expect(cb).not.toHaveBeenCalled();
		spy.mockRestore();
	});

	it('calls the callback on visibilitychange', () => {
		const cb = vi.fn();
		onPageFocus(cb);

		document.dispatchEvent(new Event('visibilitychange'));

		expect(cb).toHaveBeenCalledTimes(1);
	});

	it('cleanup removes the listeners', () => {
		const cb = vi.fn();
		const removeEventListener = vi.spyOn(window, 'removeEventListener');
		const documentRemoveEventListener = vi.spyOn(document, 'removeEventListener');

		const cleanup = onPageFocus(cb);
		cleanup();

		window.dispatchEvent(new Event('focus'));
		document.dispatchEvent(new Event('visibilitychange'));

		expect(cb).not.toHaveBeenCalled();
		expect(removeEventListener).toHaveBeenCalledWith('focus', expect.any(Function));
		expect(documentRemoveEventListener).toHaveBeenCalledWith(
			'visibilitychange',
			expect.any(Function)
		);

		removeEventListener.mockRestore();
		documentRemoveEventListener.mockRestore();
	});
});
