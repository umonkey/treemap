import { addAlert } from '$lib/api/alerts';
import { uploadSingleFile } from '$lib/api/uploads';
import { mapBus } from '$lib/buses/mapBus';
import { DEFAULT_BBOX } from '$lib/constants';
import { routes } from '$lib/routes';
import { bboxCenter } from '$lib/utils/geo';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import Page from './+page.svelte';

vi.mock('$app/navigation', () => ({
	goto: vi.fn()
}));

vi.mock('$lib/api/uploads', () => ({
	uploadSingleFile: vi.fn()
}));

vi.mock('$lib/api/alerts', () => ({
	addAlert: vi.fn()
}));

vi.mock('$lib/upload', () => ({
	solidifyFile: vi.fn(async (file: File) => file)
}));

import { goto } from '$app/navigation';

const mockedGoto = vi.mocked(goto);
const mockedUploadSingleFile = vi.mocked(uploadSingleFile);
const mockedAddAlert = vi.mocked(addAlert);

// The map starts at the midpoint of the fallback bbox.
const mapCenter = bboxCenter(DEFAULT_BBOX);

const makeFile = (name: string) => new File(['data'], name, { type: 'image/jpeg' });

const addPhoto = async (name: string) => {
	const input = screen.getByLabelText('Take a photo');
	await fireEvent.change(input, { target: { files: [makeFile(name)] } });
};

let emitSpy: ReturnType<typeof vi.spyOn>;

beforeEach(() => {
	mockedGoto.mockClear();
	mockedUploadSingleFile.mockReset();
	mockedAddAlert.mockReset();

	URL.revokeObjectURL = vi.fn();
	emitSpy = vi.spyOn(mapBus, 'emit');
});

afterEach(() => {
	cleanup();
	emitSpy.mockRestore();
});

describe('add alert page', () => {
	test('keeps Send disabled until a description and a photo are present', async () => {
		const user = userEvent.setup();

		render(Page);

		const send = screen.getByRole('button', { name: 'Send' }) as HTMLButtonElement;
		expect(send.disabled).toBe(true);

		await user.type(screen.getByLabelText('Description'), 'A fallen branch');

		expect(send.disabled).toBe(true);

		await addPhoto('branch.jpg');

		await waitFor(() => expect(send.disabled).toBe(false));
	});

	test('uploads the photos, creates the alert, and opens the preview', async () => {
		const user = userEvent.setup();

		mockedUploadSingleFile
			.mockResolvedValueOnce({ status: 200, data: 'file1', error: undefined })
			.mockResolvedValueOnce({ status: 200, data: 'file2', error: undefined });

		mockedAddAlert.mockResolvedValueOnce({
			status: 200,
			data: {
				id: 'alert1',
				lat: mapCenter.lat,
				lon: mapCenter.lng,
				created_at: 1,
				description: 'A fallen branch',
				source: 'web',
				editable: true
			},
			error: undefined
		});

		render(Page);

		await user.type(screen.getByLabelText('Description'), 'A fallen branch');

		await addPhoto('branch-1.jpg');
		await waitFor(() => expect(screen.getAllByAltText('Queued alert photo')).toHaveLength(1));

		await addPhoto('branch-2.jpg');
		await waitFor(() => expect(screen.getAllByAltText('Queued alert photo')).toHaveLength(2));

		const send = screen.getByRole('button', { name: 'Send' }) as HTMLButtonElement;
		await waitFor(() => expect(send.disabled).toBe(false));

		await user.click(send);

		await waitFor(() => expect(mockedGoto).toHaveBeenCalledWith(routes.alertPreview('alert1')));

		expect(mockedUploadSingleFile).toHaveBeenCalledTimes(2);
		expect(mockedAddAlert).toHaveBeenCalledTimes(1);
		expect(mockedAddAlert).toHaveBeenCalledWith({
			lat: mapCenter.lat,
			lon: mapCenter.lng,
			description: 'A fallen branch',
			files: ['file1', 'file2']
		});

		const uploadOrder = mockedUploadSingleFile.mock.invocationCallOrder[0];
		const alertOrder = mockedAddAlert.mock.invocationCallOrder[0];
		expect(uploadOrder).toBeLessThan(alertOrder);

		expect(mapBus.emit).toHaveBeenCalledWith('reload');
	});

	test('posts alerts to v1/alerts', async () => {
		const actual = await vi.importActual<typeof import('$lib/api/alerts')>('$lib/api/alerts');
		const fetchMock = vi.fn().mockResolvedValue({
			status: 200,
			json: async () => ({ id: 'alert1', source: 'web' })
		});

		vi.stubGlobal('fetch', fetchMock);

		const payload = {
			lat: mapCenter.lat,
			lon: mapCenter.lng,
			description: 'A fallen branch',
			files: ['1']
		};

		await actual.addAlert(payload);

		expect(fetchMock).toHaveBeenCalledTimes(1);

		const sent = fetchMock.mock.calls[0][0] as Request;
		expect(sent.method).toBe('POST');
		expect(sent.url).toBe('https://api.treemaps.app/v1/alerts');
		expect(await sent.text()).toBe(JSON.stringify(payload));
	});
});
