import { getInstance, updateInstance } from '$lib/api/instance';
import { showError } from '$lib/errors';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PageState } from './page.svelte';

vi.mock('$lib/api/instance', () => ({
	getInstance: vi.fn(),
	updateInstance: vi.fn()
}));

vi.mock('$lib/errors', () => ({
	showError: vi.fn()
}));

vi.mock('$lib/stores/settings.svelte', () => ({
	settingsState: {
		load: vi.fn()
	}
}));

const mockedGetInstance = vi.mocked(getInstance);
const mockedUpdateInstance = vi.mocked(updateInstance);
const mockedShowError = vi.mocked(showError);

describe('configure PageState', () => {
	beforeEach(() => {
		mockedGetInstance.mockReset();
		mockedUpdateInstance.mockReset();
		mockedShowError.mockReset();
	});

	it('loads OSM settings from the API', async () => {
		mockedGetInstance.mockResolvedValue({
			status: 200,
			data: {
				name: 'Example Trees',
				description: null,
				osm_pull: true,
				osm_bot_id: '134022734140280832'
			}
		});

		const state = new PageState();

		await state.reload();

		expect(state.name).toBe('Example Trees');
		expect(state.osmPull).toBe(true);
		expect(state.osmBotId).toBe('134022734140280832');
	});

	it('sends the OSM settings on apply', async () => {
		mockedUpdateInstance.mockResolvedValue({ status: 202 });

		const state = new PageState();
		state.name = 'Example Trees';
		state.description = 'A test instance.';
		state.osmPull = true;
		state.osmBotId = ' 42 ';

		await state.apply();

		expect(mockedUpdateInstance).toHaveBeenCalledWith({
			name: 'Example Trees',
			description: 'A test instance.',
			osm_pull: true,
			osm_bot_id: '42'
		});
	});

	it('sends an empty string to clear the bot id', async () => {
		mockedUpdateInstance.mockResolvedValue({ status: 202 });

		const state = new PageState();
		state.name = 'Example Trees';
		state.osmBotId = '   ';

		await state.apply();

		expect(mockedUpdateInstance).toHaveBeenCalledWith(expect.objectContaining({ osm_bot_id: '' }));
	});

	it('shows an error message when saving fails', async () => {
		mockedUpdateInstance.mockResolvedValue({
			status: 400,
			error: { code: 'BadRequest', description: 'Invalid bot id.' }
		});

		const state = new PageState();

		await state.apply();

		expect(mockedShowError).toHaveBeenCalledWith('Invalid bot id.');
	});
});
