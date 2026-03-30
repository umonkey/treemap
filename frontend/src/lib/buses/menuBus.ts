// This bus displays the tree context menu, see TreeContextMenu.

import mitt from 'mitt';

type MenuBusEvent = {
	show: string;
};

export const menuBus = mitt<MenuBusEvent>();
