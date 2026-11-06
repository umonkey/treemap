import { getAlertList, type IAlertRecord } from '$lib/api/alerts';
import type { IUser } from '$lib/types';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PageState } from './page.svelte';

vi.mock('$lib/api/alerts', () => ({
	getAlertList: vi.fn()
}));

const mockedGetAlertList = vi.mocked(getAlertList);

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

const webAlert: IAlertRecord = {
	id: '7',
	created_at: 1740304662,
	created_by: '42',
	source: 'web',
	username: null,
	language_code: null,
	lat: 40.1,
	lon: 44.5,
	description: 'A broken branch',
	status: 'draft',
	response_text: null,
	responded_at: null,
	reported_at: null,
	ping_at: null
};

const telegramAlert: IAlertRecord = {
	...webAlert,
	id: '8',
	source: 'telegram',
	created_by: '42',
	username: 'alice_tg'
};

describe('alerts list PageState', () => {
	beforeEach(() => {
		mockedGetAlertList.mockReset();
	});

	it('loads alerts and users', async () => {
		mockedGetAlertList.mockResolvedValueOnce({
			status: 200,
			data: { alerts: [webAlert], users: [user] }
		});

		const state = new PageState();

		await state.reload();

		expect(state.alerts).toEqual([webAlert]);
		expect(state.users).toEqual([user]);
		expect(state.error).toBeUndefined();
	});

	it('resolves a web reporter by id', () => {
		const state = new PageState();
		state.users = [user];

		expect(state.reporter(webAlert)).toEqual(user);
	});

	it('never matches a Telegram id against platform users', () => {
		const state = new PageState();
		state.users = [user];

		expect(state.reporter(telegramAlert)).toBeUndefined();
	});

	it('stores the API error when loading fails', async () => {
		mockedGetAlertList.mockResolvedValueOnce({
			status: 403,
			error: { code: 'Forbidden', description: 'Missing alert:manage.' }
		});

		const state = new PageState();

		await state.reload();

		expect(state.error).toEqual({ code: 'Forbidden', description: 'Missing alert:manage.' });
		expect(state.alerts).toEqual([]);
	});
});
