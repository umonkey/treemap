import { goto } from '$app/navigation';
import { getAlert, updateAlert, type IAlert } from '$lib/api/alerts';
import { mapBus } from '$lib/buses/mapBus';
import { routes } from '$lib/routes';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import Page from './+page.svelte';

vi.mock('$app/state', () => ({
	page: {
		params: { id: '1' }
	}
}));

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/api/alerts', () => ({
	getAlert: vi.fn(),
	updateAlert: vi.fn()
}));

const mockedGoto = vi.mocked(goto);
const mockedGetAlert = vi.mocked(getAlert);
const mockedUpdateAlert = vi.mocked(updateAlert);

const alert: IAlert = {
	id: '1',
	lat: 40.1,
	lon: 44.5,
	created_at: 1,
	description: 'Original text',
	source: 'web',
	editable: true
};

let emitSpy: ReturnType<typeof vi.spyOn>;

beforeEach(() => {
	mockedGoto.mockClear();
	mockedGetAlert.mockReset();
	mockedUpdateAlert.mockReset();
	emitSpy = vi.spyOn(mapBus, 'emit');
});

afterEach(() => {
	cleanup();
	emitSpy.mockRestore();
});

describe('edit alert page', () => {
	test('prefills the textarea with the alert description', async () => {
		mockedGetAlert.mockResolvedValueOnce({ status: 200, data: alert, error: undefined });

		render(Page);

		const textarea = (await screen.findByLabelText('Description')) as HTMLTextAreaElement;
		expect(textarea.value).toBe('Original text');
	});

	test('trims the description, saves it, and opens the preview', async () => {
		const user = userEvent.setup();

		mockedGetAlert.mockResolvedValueOnce({ status: 200, data: alert, error: undefined });
		mockedUpdateAlert.mockResolvedValueOnce({
			status: 200,
			data: { ...alert, description: 'trimmed text' },
			error: undefined
		});

		render(Page);

		const textarea = (await screen.findByLabelText('Description')) as HTMLTextAreaElement;
		await user.clear(textarea);
		await user.type(textarea, '  trimmed text  ');

		const save = screen.getByRole('button', { name: 'Save' }) as HTMLButtonElement;
		await waitFor(() => expect(save.disabled).toBe(false));

		await user.click(save);

		await waitFor(() =>
			expect(mockedUpdateAlert).toHaveBeenCalledWith('1', { description: 'trimmed text' })
		);
		expect(mockedGoto).toHaveBeenCalledWith(routes.alertPreview('1'));
		expect(mapBus.emit).toHaveBeenCalledWith('reload');
	});

	test('saves when Ctrl+Enter is pressed in the description', async () => {
		const user = userEvent.setup();

		mockedGetAlert.mockResolvedValueOnce({ status: 200, data: alert, error: undefined });
		mockedUpdateAlert.mockResolvedValueOnce({
			status: 200,
			data: { ...alert, description: 'updated text' },
			error: undefined
		});

		render(Page);

		const textarea = (await screen.findByLabelText('Description')) as HTMLTextAreaElement;
		await user.clear(textarea);
		await user.type(textarea, 'updated text');

		await fireEvent.keyDown(textarea, { key: 'Enter', ctrlKey: true });

		await waitFor(() =>
			expect(mockedUpdateAlert).toHaveBeenCalledWith('1', { description: 'updated text' })
		);
		expect(mockedGoto).toHaveBeenCalledWith(routes.alertPreview('1'));
	});

	test('shows an error and keeps Save disabled for a non-editable alert', async () => {
		mockedGetAlert.mockResolvedValueOnce({
			status: 200,
			data: { ...alert, editable: false },
			error: undefined
		});

		render(Page);

		expect(await screen.findByText('You are not allowed to edit this alert.')).toBeTruthy();

		const save = screen.getByRole('button', { name: 'Save' }) as HTMLButtonElement;
		expect(save.disabled).toBe(true);
	});
});
