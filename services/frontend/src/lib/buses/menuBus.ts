// This bus displays the tree context menu (see TreeContextMenu), the map context menu (see ContextMenu),
// the tree upload photo menu (see PhotoContextMenu), and the profile uploads menu (see UploadContextMenu).

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

export interface IPhotoMenuEvent {
	fileId: string;
	treeId: string;
	x?: number; // viewport-relative screen X
	y?: number; // viewport-relative screen Y
}

export interface IUploadMenuEvent {
	uploadId: number;
	x?: number; // viewport-relative screen X
	y?: number; // viewport-relative screen Y
}

type MenuBusEvent = {
	show: ITreeMenuEvent;
	showMap: IMapMenuEvent;
	showPhoto: IPhotoMenuEvent;
	showUpload: IUploadMenuEvent;
};

export const menuBus = mitt<MenuBusEvent>();
