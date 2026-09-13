// This bus displays the tree context menu (see TreeContextMenu) and the map context menu (see ContextMenu).

import type { ILatLng } from '$lib/types';
import mitt from 'mitt';

type MenuBusEvent = {
	show: string;
	showMap: ILatLng;
};

export const menuBus = mitt<MenuBusEvent>();
