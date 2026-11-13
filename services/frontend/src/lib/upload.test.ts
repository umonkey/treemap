import { addPhotos, createUploadTicket, finishUpload, putFileToStorage } from '$lib/api/uploads';
import { db, type IUpload } from '$lib/db';
import { getRetryDelay, isRetryableError, processUpload, UploadError } from '$lib/upload';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

vi.mock('$lib/api/uploads', () => ({
	addPhotos: vi.fn(),
	createUploadTicket: vi.fn(),
	finishUpload: vi.fn(),
	putFileToStorage: vi.fn()
}));

// Keep the module's import-time auto-start from processing the queue while the
// tests drive processUpload directly.
vi.mock('$lib/stores/upload', async () => {
	const { writable } = await import('svelte/store');

	return {
		uploadStore: writable({ pending: 0, autoupload: false, uploading: false }),
		setUploading: vi.fn()
	};
});

const makeRecord = (overrides: Partial<IUpload> = {}): IUpload => ({
	tree_id: 'tree-1',
	status: 'pending',
	image: new Blob(['photo'], { type: 'image/jpeg' }),
	created_at: Date.now(),
	retry_count: 0,
	file_id: null,
	progress: 0,
	stage: 'new',
	...overrides
});

const addRecord = async (overrides: Partial<IUpload> = {}): Promise<IUpload> => {
	const record = makeRecord(overrides);
	const id = await db.uploads.add(record);
	const item = await db.uploads.get(id);

	if (!item) {
		throw new Error('Failed to create an upload record');
	}

	// fake-indexeddb does not round-trip jsdom Blobs: size and type come back
	// undefined and empty. Keep the in-memory Blob, which is what the browser
	// provides, while still exercising the persisted record.
	return { ...item, image: record.image };
};

beforeEach(async () => {
	vi.resetAllMocks();
	await db.uploads.clear();
});

afterEach(async () => {
	vi.restoreAllMocks();
	await db.uploads.clear();
});

describe('processUpload', () => {
	test('resumes at the uploaded stage without transferring the file again', async () => {
		const item = await addRecord({ stage: 'uploaded', file_id: 'file-1' });
		vi.mocked(finishUpload).mockResolvedValue({ status: 200 });
		vi.mocked(addPhotos).mockResolvedValue({ status: 200 });

		await expect(processUpload(item)).resolves.toBe('file-1');

		expect(finishUpload).toHaveBeenCalledTimes(1);
		expect(finishUpload).toHaveBeenCalledWith('file-1');
		expect(addPhotos).toHaveBeenCalledTimes(1);
		expect(addPhotos).toHaveBeenCalledWith('tree-1', ['file-1']);
		expect(createUploadTicket).not.toHaveBeenCalled();
		expect(putFileToStorage).not.toHaveBeenCalled();
	});

	test('only attaches the file at the finished stage', async () => {
		const item = await addRecord({ stage: 'finished', file_id: 'file-1' });
		vi.mocked(addPhotos).mockResolvedValue({ status: 200 });

		await expect(processUpload(item)).resolves.toBe('file-1');

		expect(addPhotos).toHaveBeenCalledTimes(1);
		expect(addPhotos).toHaveBeenCalledWith('tree-1', ['file-1']);
		expect(createUploadTicket).not.toHaveBeenCalled();
		expect(putFileToStorage).not.toHaveBeenCalled();
		expect(finishUpload).not.toHaveBeenCalled();
	});

	test('runs the whole pipeline for a new item and persists every stage', async () => {
		const item = await addRecord({ stage: 'new' });
		const onProgress = vi.fn();

		vi.mocked(createUploadTicket).mockResolvedValue({
			status: 200,
			data: { id: 'file-2', url: 'https://storage.test/put' }
		});
		vi.mocked(putFileToStorage).mockResolvedValue({ status: 200 });
		vi.mocked(finishUpload).mockResolvedValue({ status: 200 });
		vi.mocked(addPhotos).mockResolvedValue({ status: 200 });

		await expect(processUpload(item, onProgress)).resolves.toBe('file-2');

		expect(createUploadTicket).toHaveBeenCalledWith(item.image.size, 'image/jpeg');
		expect(putFileToStorage).toHaveBeenCalledWith(
			'https://storage.test/put',
			item.image,
			onProgress
		);
		expect(finishUpload).toHaveBeenCalledWith('file-2');
		expect(addPhotos).toHaveBeenCalledWith('tree-1', ['file-2']);

		const updated = await db.uploads.get(item.id as number);
		expect(updated?.stage).toBe('finished');
		expect(updated?.file_id).toBe('file-2');
	});

	test('keeps the uploaded stage and file id when finishing fails', async () => {
		const item = await addRecord({ stage: 'uploaded', file_id: 'file-1' });
		vi.mocked(finishUpload).mockResolvedValue({
			status: 500,
			error: { code: 'http_error', description: 'database is locked' }
		});

		await expect(processUpload(item)).rejects.toMatchObject({ status: 500 });

		const updated = await db.uploads.get(item.id as number);
		expect(updated?.stage).toBe('uploaded');
		expect(updated?.file_id).toBe('file-1');
		expect(putFileToStorage).not.toHaveBeenCalled();
	});

	test('marks a failed attachment as non-retryable', async () => {
		const item = await addRecord({ stage: 'finished', file_id: 'file-1' });
		vi.mocked(addPhotos).mockResolvedValue({
			status: 404,
			error: { code: 'not_found', description: 'Tree not found' }
		});

		const error = await processUpload(item).catch((e: unknown) => e);

		expect(error).toBeInstanceOf(UploadError);

		const uploadError = error as UploadError;
		expect(uploadError.stage).toBe('finished');
		expect(isRetryableError(uploadError.status, uploadError.code)).toBe(false);
	});

	test('backs off exponentially with jitter and a cap', () => {
		const random = vi.spyOn(Math, 'random').mockReturnValue(0.5);
		const delays = [1, 2, 3, 4, 5, 6, 10].map((attempt) => getRetryDelay(attempt));
		random.mockRestore();

		for (let i = 1; i < delays.length; i += 1) {
			expect(delays[i]).toBeGreaterThanOrEqual(delays[i - 1]);
		}

		expect(delays).toEqual([5000, 10000, 20000, 40000, 60000, 60000, 60000]);

		// Jitter is plus or minus twenty percent.
		vi.spyOn(Math, 'random').mockReturnValue(0);
		expect(getRetryDelay(1)).toBe(4000);

		vi.spyOn(Math, 'random').mockReturnValue(1);
		expect(getRetryDelay(1)).toBe(6000);
	});
});
