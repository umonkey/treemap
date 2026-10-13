import { cleanup, fireEvent, render } from '@testing-library/svelte';
import { afterEach, describe, expect, test, vi } from 'vitest';
import type { IGalleryItem } from '$lib/types';
import ImageViewer from './ImageViewer.svelte';
import { ImageViewerLogic } from './ImageViewer.svelte.ts';

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

// jsdom has neither PointerEvent nor setPointerCapture, so pinch/drag/tap
// behavior is exercised against the logic class directly.
const fakeImage = () => ({ clientWidth: 800, clientHeight: 600 }) as unknown as HTMLImageElement;

const pointer = (id: number, x: number, y: number, pointerType = 'mouse') =>
	({
		pointerId: id,
		clientX: x,
		clientY: y,
		pointerType,
		currentTarget: { setPointerCapture: () => {} }
	}) as unknown as PointerEvent;

const wheel = (deltaY: number, x: number, y: number) =>
	({ deltaY, clientX: x, clientY: y, preventDefault: () => {} }) as unknown as WheelEvent;

const setViewport = (width: number, height: number) => {
	Object.defineProperty(window, 'innerWidth', { configurable: true, value: width });
	Object.defineProperty(window, 'innerHeight', { configurable: true, value: height });
};

describe('ImageViewerLogic', () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	test('wheel up zooms in and wheel down clamps at 1x', () => {
		setViewport(1024, 768);
		const viewer = new ImageViewerLogic();
		viewer.bindImage(fakeImage());

		viewer.handleWheel(wheel(-100, 512, 384));
		expect(viewer.scale).toBeGreaterThan(1);

		for (let i = 0; i < 50; i++) viewer.handleWheel(wheel(1000, 512, 384));
		expect(viewer.scale).toBe(1);
		expect(viewer.offsetX).toBe(0);
		expect(viewer.offsetY).toBe(0);
	});

	test('zoom keeps the local coordinate under the focal point', () => {
		setViewport(1000, 800);
		const viewer = new ImageViewerLogic();
		viewer.bindImage({ clientWidth: 2000, clientHeight: 2000 } as unknown as HTMLImageElement);

		const cx = window.innerWidth / 2;
		const cy = window.innerHeight / 2;
		const focalX = 600;
		const focalY = 400;
		const before = {
			x: (focalX - cx - viewer.offsetX) / viewer.scale,
			y: (focalY - cy - viewer.offsetY) / viewer.scale
		};

		viewer.handleDblClick({ clientX: focalX, clientY: focalY } as MouseEvent);
		expect(viewer.scale).toBe(2);

		const after = {
			x: (focalX - cx - viewer.offsetX) / viewer.scale,
			y: (focalY - cy - viewer.offsetY) / viewer.scale
		};
		expect(after.x).toBeCloseTo(before.x);
		expect(after.y).toBeCloseTo(before.y);
	});

	test('pan is clamped to the scaled image bounds', () => {
		setViewport(1024, 768);
		const viewer = new ImageViewerLogic();
		viewer.bindImage(fakeImage());

		viewer.handleDblClick({ clientX: 512, clientY: 384 } as MouseEvent);
		expect(viewer.scale).toBe(2);

		viewer.handlePointerDown(pointer(1, 500, 500));
		viewer.handlePointerMove(pointer(1, 5000, 5000));

		expect(viewer.offsetX).toBe((800 * 2 - 1024) / 2);
		expect(viewer.offsetY).toBe((600 * 2 - 768) / 2);
	});

	test('single-pointer drag pans when zoomed', () => {
		setViewport(1024, 768);
		const viewer = new ImageViewerLogic();
		viewer.bindImage(fakeImage());

		viewer.handleDblClick({ clientX: 512, clientY: 384 } as MouseEvent);
		viewer.handlePointerDown(pointer(1, 500, 500));
		viewer.handlePointerMove(pointer(1, 550, 530));

		expect(viewer.offsetX).toBe(50);
		expect(viewer.offsetY).toBe(30);
		expect(viewer.dragging).toBe(true);
	});

	test('single-pointer drag does not pan at 1x', () => {
		setViewport(1024, 768);
		const viewer = new ImageViewerLogic();
		viewer.bindImage(fakeImage());

		viewer.handlePointerDown(pointer(1, 500, 500));
		viewer.handlePointerMove(pointer(1, 550, 530));

		expect(viewer.offsetX).toBe(0);
		expect(viewer.offsetY).toBe(0);
		expect(viewer.dragging).toBe(false);
	});

	test('pinch with two pointers moving apart zooms in', () => {
		setViewport(1024, 768);
		const viewer = new ImageViewerLogic();
		viewer.bindImage(fakeImage());

		viewer.handlePointerDown(pointer(1, 400, 400, 'touch'));
		viewer.handlePointerDown(pointer(2, 500, 400, 'touch'));
		viewer.handlePointerMove(pointer(2, 700, 400, 'touch'));

		expect(viewer.scale).toBeCloseTo(3);
	});

	test('double-tap toggles between 1x and 2x', () => {
		setViewport(1024, 768);
		let now = 10_000;
		vi.spyOn(performance, 'now').mockImplementation(() => now);

		const viewer = new ImageViewerLogic();
		viewer.bindImage(fakeImage());

		const tap = () => {
			viewer.handlePointerDown(pointer(1, 512, 384, 'touch'));
			viewer.handlePointerUp(pointer(1, 512, 384, 'touch'));
		};

		tap();
		expect(viewer.scale).toBe(1);

		now += 100;
		tap();
		expect(viewer.scale).toBe(2);

		now += 100;
		tap();
		expect(viewer.scale).toBe(2);

		now += 100;
		tap();
		expect(viewer.scale).toBe(1);
	});

	test('reset restores the fit view', () => {
		setViewport(1024, 768);
		const viewer = new ImageViewerLogic();
		viewer.bindImage(fakeImage());

		viewer.handleDblClick({ clientX: 600, clientY: 300 } as MouseEvent);
		viewer.reset();

		expect(viewer.scale).toBe(1);
		expect(viewer.offsetX).toBe(0);
		expect(viewer.offsetY).toBe(0);
		expect(viewer.dragging).toBe(false);
	});
});
