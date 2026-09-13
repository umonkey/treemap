// This bus displays the tree context menu (see TreeContextMenu) and the map context menu (see ContextMenu).

import mitt from 'mitt';

export interface IMapMenuEvent {
	lat: number;
	lng: number;
	x?: number; // viewport-relative screen X
	y?: number; // viewport-relative screen Y
}

type MenuBusEvent = {
	show: string;
	showMap: IMapMenuEvent;
};

export const menuBus = mitt<MenuBusEvent>();
