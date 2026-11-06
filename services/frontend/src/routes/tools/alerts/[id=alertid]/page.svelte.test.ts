import { getAlertFull, getAlertPhotos, type IAlertRecord } from '$lib/api/alerts';
import type { IUser } from '$lib/types';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PageState } from './page.svelte';

vi.mock('$lib/api/alerts', () => ({
	getAlertFull: vi.fn(),
	getAlertPhotos: vi.fn()
}));

const mockedGetAlertFull = vi.mocked(getAlertFull);
const mockedGetAlertPhotos = vi.mocked(getAlertPhotos);

const user: IUser = {
	id: '42',
	name: 'Alice',
	email: 'alice@example.com',
	picture: '',
	trees_count: 0,
	comments_count: 0,
	updates_count: 0,
	files_count: 0,
	last_active_at: 0
};

const alert: IAlertRecord = {
	id: '7',
	created_at: 1740304662,
	created_by: '42',
	source: 'web',
	username: null,
	language_code: null,
	lat: 55.75,
	lon: 37.61,
	description: 'A broken tree',
	status: 'processed',
	response_text: 'Thank you.',
	responded_at: 1740304800,
	reported_at: 1740304660,
	ping_at: null
};

describe('alert detail PageState', () => {
	beforeEach(() => {
		mockedGetAlertFull.mockReset();
		mockedGetAlertPhotos.mockReset();
		mockedGetAlertPhotos.mockResolvedValue({ status: 200, data: [] });
	});

	it('loads the alert, users, and photos', async () => {
		mockedGetAlertFull.mockResolvedValueOnce({
			status: 200,
			data: { alert, users: [user] }
		});
		mockedGetAlertPhotos.mockResolvedValueOnce({
			status: 200,
			data: ['https://example.com/photo.jpg']
		});

		const state = new PageState();

		await state.reload('7');

		expect(state.alert).toEqual(alert);
		expect(state.users).toEqual([user]);
		expect(state.photos).toEqual(['https://example.com/photo.jpg']);
		expect(state.error).toBeUndefined();
	});

	it('stores a 404 error and clears the alert', async () => {
		mockedGetAlertFull.mockResolvedValueOnce({
			status: 404,
			error: { code: 'NotFound', description: 'Alert not found.' }
		});

		const state = new PageState();

		await state.reload('404');

		expect(state.error).toEqual({ code: 'NotFound', description: 'Alert not found.' });
		expect(state.alert).toBeUndefined();
		expect(state.users).toEqual([]);
	});

	it('keeps the alert visible when photos fail', async () => {
		mockedGetAlertFull.mockResolvedValueOnce({
			status: 200,
			data: { alert, users: [user] }
		});
		mockedGetAlertPhotos.mockResolvedValueOnce({
			status: 500,
			error: { code: 'ServerError', description: 'Photos failed.' }
		});

		const state = new PageState();

		await state.reload('7');

		expect(state.alert).toEqual(alert);
		expect(state.error).toBeUndefined();
		expect(state.photos).toEqual([]);
	});

	it('never matches a Telegram id against platform users', () => {
		const state = new PageState();
		state.users = [user];

		expect(state.reporter({ ...alert, source: 'telegram' })).toBeUndefined();
	});
});
