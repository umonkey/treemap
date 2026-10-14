import { queuePhoto } from '$lib/upload';
import { beforeEach, describe, expect, test, vi } from 'vitest';
import { ClipboardUploader } from './ClipboardUploader.svelte.ts';

const auth = vi.hoisted(() => ({ allowed: true }));

vi.mock('$lib/upload', () => ({
	queuePhoto: vi.fn()
}));

vi.mock('$lib/stores/authStore', async () => {
	const { writable } = await import('svelte/store');
	return {
		hasPermission: writable(() => auth.allowed)
	};
});

const mockedQueuePhoto = vi.mocked(queuePhoto);

const photo = (name = 'photo.png', type = 'image/png') => new File(['x'], name, { type });

interface FakeItem {
	kind: string;
	type: string;
	file: File | null;
}

const paste = (items: FakeItem[]) => {
	const preventDefault = vi.fn();
	const event = {
		clipboardData: {
			items: items.map((item) => ({
				kind: item.kind,
				type: item.type,
				getAsFile: () => item.file
			}))
		},
		preventDefault
	} as unknown as ClipboardEvent;

	return { event, preventDefault };
};

describe('ClipboardUploader', () => {
	beforeEach(() => {
		auth.allowed = true;
		mockedQueuePhoto.mockReset();
	});

	test('queues an image pasted from the clipboard', async () => {
		const file = photo();
		const { event, preventDefault } = paste([{ kind: 'file', type: 'image/png', file }]);
		const logic = new ClipboardUploader();

		await logic.handlePaste('123', event);

		expect(mockedQueuePhoto).toHaveBeenCalledTimes(1);
		expect(mockedQueuePhoto).toHaveBeenCalledWith('123', file);
		expect(preventDefault).toHaveBeenCalledTimes(1);
	});

	test('queues every image in a multi-image paste', async () => {
		const first = photo('a.png');
		const second = photo('b.jpg', 'image/jpeg');
		const { event, preventDefault } = paste([
			{ kind: 'file', type: 'image/png', file: first },
			{ kind: 'file', type: 'image/jpeg', file: second }
		]);
		const logic = new ClipboardUploader();

		await logic.handlePaste('123', event);

		expect(mockedQueuePhoto).toHaveBeenCalledTimes(2);
		expect(mockedQueuePhoto).toHaveBeenNthCalledWith(1, '123', first);
		expect(mockedQueuePhoto).toHaveBeenNthCalledWith(2, '123', second);
		expect(preventDefault).toHaveBeenCalledTimes(1);
	});

	test('ignores text and non-image clipboard payloads', async () => {
		const { event, preventDefault } = paste([
			{ kind: 'string', type: 'text/plain', file: null },
			{ kind: 'file', type: 'application/pdf', file: photo('doc.pdf', 'application/pdf') }
		]);
		const logic = new ClipboardUploader();

		await logic.handlePaste('123', event);

		expect(mockedQueuePhoto).not.toHaveBeenCalled();
		expect(preventDefault).not.toHaveBeenCalled();
	});

	test('does nothing without the tree:edit permission', async () => {
		auth.allowed = false;
		const { event, preventDefault } = paste([{ kind: 'file', type: 'image/png', file: photo() }]);
		const logic = new ClipboardUploader();

		await logic.handlePaste('123', event);

		expect(mockedQueuePhoto).not.toHaveBeenCalled();
		expect(preventDefault).not.toHaveBeenCalled();
	});
});
