import { describe, it, expect, beforeEach, vi } from 'vitest';
import { ContextMenuLogic } from './ContextMenu.svelte.ts';
import { menuBus } from '$lib/buses/menuBus';

vi.mock('$lib/routes', () => ({
	goto: vi.fn().mockResolvedValue(undefined)
}));

describe('ContextMenuLogic', () => {
	let logic: ContextMenuLogic;

	beforeEach(() => {
		vi.clearAllMocks();
		logic = new ContextMenuLogic();
	});

	it('closes an open menu on a real right-click and prevents the default', () => {
		logic.open = true;
		const event = new MouseEvent('contextmenu', { button: 2, cancelable: true });
		const preventDefault = vi.spyOn(event, 'preventDefault');

		logic.handleContextMenu(event);

		expect(preventDefault).toHaveBeenCalled();
		expect(logic.open).toBe(false);
	});

	it('keeps the menu open on a touch long-press (button 0) but still prevents the default', () => {
		logic.open = true;
		const event = new MouseEvent('contextmenu', { button: 0, cancelable: true });
		const preventDefault = vi.spyOn(event, 'preventDefault');

		logic.handleContextMenu(event);

		expect(preventDefault).toHaveBeenCalled();
		expect(logic.open).toBe(true);
	});

	it('closes the menu when Escape is pressed', () => {
		logic.open = true;

		logic.handleKeyDown(new KeyboardEvent('keydown', { key: 'Escape' }));

		expect(logic.open).toBe(false);
	});

	it('opens the menu and clamps the position on a showMap event', () => {
		logic.viewport = { width: 1000, height: 800 };
		logic.menuSize = { width: 200, height: 100 };

		const cleanup = logic.subscribe();
		menuBus.emit('showMap', { lat: 40.1, lng: 44.5, x: 100, y: 200 });

		expect(logic.open).toBe(true);
		expect(logic.clampedPosition).toEqual({ x: 100, y: 200 });

		cleanup();
	});
});
