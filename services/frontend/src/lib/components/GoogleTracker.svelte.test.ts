import { beforeEach, describe, expect, it, vi } from 'vitest';
import { GoogleTrackerLogic } from './GoogleTracker.svelte.ts';

vi.mock('$app/environment', () => ({
	browser: true
}));

describe('GoogleTrackerLogic', () => {
	let logic: GoogleTrackerLogic;
	let gtag: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		logic = new GoogleTrackerLogic();
		logic.gaId = 'G-TEST';

		window.dataLayer = [];

		gtag = vi.fn();
		vi.stubGlobal('gtag', gtag);
	});

	it('passes a stored user id to the initial GA config', () => {
		logic.init('user-1');

		const config = window.dataLayer.find((entry) => entry[0] === 'config');

		expect(config?.[1]).toBe('G-TEST');
		expect(config?.[2]).toMatchObject({
			send_page_view: false,
			user_id: 'user-1'
		});
	});

	it('does not re-send the user id when it is unchanged', () => {
		logic.init('user-1');
		const length = window.dataLayer.length;

		logic.setUserId('user-1');

		expect(window.dataLayer).toHaveLength(length);
	});

	it('sets the user id once per change', () => {
		logic.setUserId('user-1');
		logic.setUserId('user-1');
		logic.setUserId('user-2');

		expect(gtag).toHaveBeenCalledTimes(2);
		expect(gtag).toHaveBeenNthCalledWith(1, 'set', { user_id: 'user-1' });
		expect(gtag).toHaveBeenNthCalledWith(2, 'set', { user_id: 'user-2' });
	});

	it('clears the user id on sign-out', () => {
		logic.setUserId('user-1');
		logic.setUserId(undefined);

		expect(gtag).toHaveBeenLastCalledWith('set', { user_id: null });
	});

	it('tracks page views', () => {
		logic.trackPageView('https://yerevan.treemaps.app/search?q=oak', '/search?q=oak', 'Search');

		expect(gtag).toHaveBeenCalledWith('event', 'page_view', {
			page_title: 'Search',
			page_location: 'https://yerevan.treemaps.app/search?q=oak',
			page_path: '/search?q=oak'
		});
	});
});
