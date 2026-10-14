import { queuePhoto } from '$lib/upload';
import { beforeEach, describe, expect, test, vi } from 'vitest';
import { Gallery } from './Gallery.svelte.ts';

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

const dragEvent = (files: File[] = [], types: string[] = ['Files']) => {
	const preventDefault = vi.fn();

	return {
		event: {
			dataTransfer: { files, types, dropEffect: '' },
			preventDefault
		} as unknown as DragEvent,
		preventDefault
	};
};

describe('Gallery', () => {
	beforeEach(() => {
		auth.allowed = true;
		mockedQueuePhoto.mockReset();
	});

	test('queues an image dropped onto the gallery', async () => {
		const file = photo();
		const { event, preventDefault } = dragEvent([file]);
		const gallery = new Gallery();

		await gallery.handleDrop('123', event);

		expect(mockedQueuePhoto).toHaveBeenCalledTimes(1);
		expect(mockedQueuePhoto).toHaveBeenCalledWith('123', file);
		expect(preventDefault).toHaveBeenCalledTimes(1);
	});

	test('queues every image in a multi-image drop', async () => {
		const first = photo('a.png');
		const second = photo('b.jpg', 'image/jpeg');
		const { event, preventDefault } = dragEvent([first, second]);
		const gallery = new Gallery();

		await gallery.handleDrop('123', event);

		expect(mockedQueuePhoto).toHaveBeenCalledTimes(2);
		expect(mockedQueuePhoto).toHaveBeenNthCalledWith(1, '123', first);
		expect(mockedQueuePhoto).toHaveBeenNthCalledWith(2, '123', second);
		expect(preventDefault).toHaveBeenCalledTimes(1);
	});

	test('ignores non-image files', async () => {
		const { event } = dragEvent([photo('doc.pdf', 'application/pdf')]);
		const gallery = new Gallery();

		await gallery.handleDrop('123', event);

		expect(mockedQueuePhoto).not.toHaveBeenCalled();
	});

	test('does nothing without the tree:edit permission', async () => {
		auth.allowed = false;
		const { event, preventDefault } = dragEvent([photo()]);
		const gallery = new Gallery();

		await gallery.handleDrop('123', event);

		expect(mockedQueuePhoto).not.toHaveBeenCalled();
		expect(preventDefault).not.toHaveBeenCalled();
	});

	test('tracks the drag state with a depth counter', () => {
		const gallery = new Gallery();
		const first = dragEvent().event;
		const second = dragEvent().event;

		expect(gallery.dragging).toBe(false);

		gallery.handleDragEnter(first);
		gallery.handleDragEnter(second);
		expect(gallery.dragging).toBe(true);

		// Leaving once only decrements the depth; the highlight stays until every
		// entered child has been left.
		gallery.handleDragLeave();
		expect(gallery.dragging).toBe(true);

		gallery.handleDragLeave();
		expect(gallery.dragging).toBe(false);
	});

	test('resets the drag state when a drop happens', async () => {
		const gallery = new Gallery();

		gallery.handleDragEnter(dragEvent().event);
		expect(gallery.dragging).toBe(true);

		await gallery.handleDrop('123', dragEvent([photo()]).event);
		expect(gallery.dragging).toBe(false);
	});

	test('ignores non-file drags', () => {
		const gallery = new Gallery();
		const { event, preventDefault } = dragEvent([], []);

		gallery.handleDragOver(event);
		gallery.handleDragEnter(event);

		expect(preventDefault).not.toHaveBeenCalled();
		expect(gallery.dragging).toBe(false);
	});

	test('highlights on dragover when dragenter had no file types', () => {
		const gallery = new Gallery();

		// Firefox can report an empty types list on the first dragenter.
		const enter = dragEvent([], []).event;
		gallery.handleDragEnter(enter);
		expect(gallery.dragging).toBe(false);

		const over = dragEvent().event;
		gallery.handleDragOver(over);

		expect(gallery.dragging).toBe(true);
	});

	test('does not highlight on a non-file dragover', () => {
		const gallery = new Gallery();
		const { event, preventDefault } = dragEvent([], []);

		gallery.handleDragOver(event);

		expect(preventDefault).not.toHaveBeenCalled();
		expect(gallery.dragging).toBe(false);
	});
});
