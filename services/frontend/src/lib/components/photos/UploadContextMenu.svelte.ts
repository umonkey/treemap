import { menuBus, type IUploadMenuEvent } from '$lib/buses/menuBus';
import type { IContextMenuPosition } from '$lib/components/menu/BaseContextMenu.svelte.ts';
import { db } from '$lib/db';

export class UploadContextMenuLogic {
	open = $state(false);
	position = $state<IContextMenuPosition | undefined>(undefined);
	private uploadId: number | undefined = undefined;

	private handleShow = (event: IUploadMenuEvent) => {
		this.uploadId = event.uploadId;
		this.position =
			event.x !== undefined && event.y !== undefined ? { x: event.x, y: event.y } : undefined;
		this.open = true;
	};

	public handleClose = () => {
		this.open = false;
	};

	public handleDelete = async () => {
		this.open = false;

		if (this.uploadId === undefined) {
			return;
		}

		await db.uploads.delete(this.uploadId);
	};

	public subscribe = () => {
		menuBus.on('showUpload', this.handleShow);

		return () => {
			menuBus.off('showUpload', this.handleShow);
		};
	};
}
