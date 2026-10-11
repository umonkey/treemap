import { describe, it, expect, beforeEach, vi } from 'vitest';
import { ContextMenuLogic } from './ContextMenu.svelte.ts';
import { mapBus } from '$lib/buses/mapBus';
import { menuBus } from '$lib/buses/menuBus';
import { searchStore } from '$lib/stores/searchStore';

vi.mock('$lib/routes', () => ({
	goto: vi.fn().mockResolvedValue(undefined)
}));

describe('ContextMenuLogic', () => {
	let logic: ContextMenuLogic;

	beforeEach(() => {
		vi.clearAllMocks();
		logic = new ContextMenuLogic();
	});

	it('opens the menu and stores the event position on a showMap event', () => {
		const cleanup = logic.subscribe();
		menuBus.emit('showMap', { lat: 40.1, lng: 44.5, x: 100, y: 200 });

		expect(logic.open).toBe(true);
		expect(logic.position).toEqual({ x: 100, y: 200 });

		cleanup();
	});

	it('opens the menu without a position (centered) when the event has no screen coordinates', () => {
		const cleanup = logic.subscribe();
		menuBus.emit('showMap', { lat: 40.1, lng: 44.5 });

		expect(logic.open).toBe(true);
		expect(logic.position).toBeUndefined();

		cleanup();
	});

	it('builds a GeoJSON download URL from the viewport and search query', () => {
		searchStore.set('liriodendron');

		const cleanup = logic.subscribe();
		mapBus.emit('bounds', { n: 41, e: 45, s: 40, w: 44, zoom: 16 });

		const url = logic.downloadUrl;
		expect(url).toContain('v1/trees/search.json');
		expect(url).toContain('n=41');
		expect(url).toContain('e=45');
		expect(url).toContain('s=40');
		expect(url).toContain('w=44');
		expect(url).toContain('zoom=16');
		expect(url).toContain('search=liriodendron');

		cleanup();
		searchStore.set(undefined);
	});

	it('closes the menu when a download starts', () => {
		logic.open = true;

		logic.handleDownload();

		expect(logic.open).toBe(false);
	});
});
