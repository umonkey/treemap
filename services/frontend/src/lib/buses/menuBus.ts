// This bus displays the tree context menu (see TreeContextMenu) and the map context menu (see ContextMenu).

import mitt from 'mitt';

export interface IMapMenuEvent {
	lat: number;
	lng: number;
	x?: number; // viewport-relative screen X
	y?: number; // viewport-relative screen Y
}

export interface ITreeMenuEvent {
	id: string;
	x?: number; // viewport-relative screen X
	y?: number; // viewport-relative screen Y
}

type MenuBusEvent = {
	show: ITreeMenuEvent;
	showMap: IMapMenuEvent;
};

export const menuBus = mitt<MenuBusEvent>();
