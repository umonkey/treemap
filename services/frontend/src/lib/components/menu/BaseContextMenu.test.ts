import { describe, it, expect, beforeEach, vi } from 'vitest';
import { BaseContextMenuLogic } from './BaseContextMenu.svelte.ts';

describe('BaseContextMenuLogic', () => {
	let logic: BaseContextMenuLogic;

	beforeEach(() => {
		logic = new BaseContextMenuLogic();
	});

	it('clamps the position inside the viewport margins', () => {
		logic.viewport = { width: 1000, height: 800 };
		logic.menuSize = { width: 200, height: 100 };
		logic.position = { x: 1500, y: 1200 };

		expect(logic.clampedPosition).toEqual({ x: 792, y: 692 });
	});

	it('clamps the position to the minimum margin', () => {
		logic.viewport = { width: 1000, height: 800 };
		logic.menuSize = { width: 200, height: 100 };
		logic.position = { x: -50, y: -50 };

		expect(logic.clampedPosition).toEqual({ x: 8, y: 8 });
	});

	it('keeps a position already inside the viewport unchanged', () => {
		logic.viewport = { width: 1000, height: 800 };
		logic.menuSize = { width: 200, height: 100 };
		logic.position = { x: 100, y: 200 };

		expect(logic.clampedPosition).toEqual({ x: 100, y: 200 });
	});

	it('returns undefined when no position is set', () => {
		logic.viewport = { width: 1000, height: 800 };
		logic.menuSize = { width: 200, height: 100 };

		expect(logic.clampedPosition).toBeUndefined();
	});

	it('calls onClose when Escape is pressed', () => {
		const onClose = vi.fn();

		logic.handleKeyDown(new KeyboardEvent('keydown', { key: 'Escape' }), onClose);

		expect(onClose).toHaveBeenCalled();
	});

	it('ignores non-Escape keys', () => {
		const onClose = vi.fn();

		logic.handleKeyDown(new KeyboardEvent('keydown', { key: 'Enter' }), onClose);

		expect(onClose).not.toHaveBeenCalled();
	});

	it('closes the menu on a real right-click and prevents the default', () => {
		const event = new MouseEvent('contextmenu', { button: 2, cancelable: true });
		const preventDefault = vi.spyOn(event, 'preventDefault');
		const onClose = vi.fn();

		logic.handleBackdropContextMenu(event, onClose);

		expect(preventDefault).toHaveBeenCalled();
		expect(onClose).toHaveBeenCalled();
	});

	it('keeps the menu open on a touch long-press (button 0) but still prevents the default', () => {
		const event = new MouseEvent('contextmenu', { button: 0, cancelable: true });
		const preventDefault = vi.spyOn(event, 'preventDefault');
		const onClose = vi.fn();

		logic.handleBackdropContextMenu(event, onClose);

		expect(preventDefault).toHaveBeenCalled();
		expect(onClose).not.toHaveBeenCalled();
	});
});
