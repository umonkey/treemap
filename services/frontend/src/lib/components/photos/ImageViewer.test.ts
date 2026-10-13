import { cleanup, fireEvent, render } from '@testing-library/svelte';
import { afterEach, describe, expect, test, vi } from 'vitest';
import type { IGalleryItem } from '$lib/types';
import ImageViewer from './ImageViewer.svelte';

const item: IGalleryItem = {
	id: '1',
	small: 'small.jpg',
	large: 'large.jpg',
	label: 'Tree'
};

describe('ImageViewer', () => {
	afterEach(cleanup);

	test('calls onClose when Escape is pressed', async () => {
		const onClose = vi.fn();

		render(ImageViewer, { props: { item, onClose } });

		await fireEvent.keyDown(window, { key: 'Escape' });

		expect(onClose).toHaveBeenCalledTimes(1);
	});

	test('calls onClose when the backdrop is clicked', async () => {
		const onClose = vi.fn();
		const { getAllByLabelText } = render(ImageViewer, { props: { item, onClose } });

		await fireEvent.click(getAllByLabelText('Close')[0]);

		expect(onClose).toHaveBeenCalledTimes(1);
	});

	test('calls onClose when the close button is clicked', async () => {
		const onClose = vi.fn();
		const { getAllByLabelText } = render(ImageViewer, { props: { item, onClose } });

		await fireEvent.click(getAllByLabelText('Close')[1]);

		expect(onClose).toHaveBeenCalledTimes(1);
	});
});
