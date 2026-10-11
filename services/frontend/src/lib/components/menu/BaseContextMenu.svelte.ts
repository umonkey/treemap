import type { Component } from 'svelte';

export interface IContextMenuItem {
	icon: Component;
	label: string;
	href?: string; // renders an <a href>, otherwise a <button>
	download?: boolean; // adds `download` to the <a>
	onClick?: () => void; // used by <button>; can also accompany an <a>
	disabled?: boolean; // <button disabled>
	separator?: boolean; // draws a top border
}

export interface IContextMenuPosition {
	x: number;
	y: number;
}

const MENU_MARGIN = 8;

export class BaseContextMenuLogic {
	viewport = $state({ width: 0, height: 0 });
	menuSize = $state({ width: 0, height: 0 });
	position = $state<IContextMenuPosition | undefined>(undefined);

	clampedPosition = $derived.by(() => {
		if (!this.position) {
			return undefined;
		}

		const maxX = Math.max(MENU_MARGIN, this.viewport.width - this.menuSize.width - MENU_MARGIN);
		const maxY = Math.max(MENU_MARGIN, this.viewport.height - this.menuSize.height - MENU_MARGIN);

		return {
			x: Math.min(Math.max(this.position.x, MENU_MARGIN), maxX),
			y: Math.min(Math.max(this.position.y, MENU_MARGIN), maxY)
		};
	});

	public handleKeyDown = (e: KeyboardEvent, onClose: () => void) => {
		if (e.key === 'Escape') {
			onClose();
		}
	};

	public handleBackdropContextMenu = (e: MouseEvent, onClose: () => void) => {
		e.preventDefault();

		// Touch long-press also emits `contextmenu` (button 0). Only a real
		// mouse right-click should dismiss the menu.
		if (e.button !== 2) {
			return;
		}

		onClose();
	};
}
