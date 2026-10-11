import { changeTreeThumbnail } from '$lib/api/trees';
import { deleteFile } from '$lib/api/uploads';
import { menuBus, type IPhotoMenuEvent } from '$lib/buses/menuBus';
import type { IContextMenuPosition } from '$lib/components/menu/BaseContextMenu.svelte.ts';
import { showError } from '$lib/errors';

export class PhotoContextMenuLogic {
	open = $state(false);
	position = $state<IContextMenuPosition | undefined>(undefined);
	private fileId: string | undefined = undefined;
	private treeId: string | undefined = undefined;

	private handleShow = (event: IPhotoMenuEvent) => {
		this.fileId = event.fileId;
		this.treeId = event.treeId;
		this.position =
			event.x !== undefined && event.y !== undefined ? { x: event.x, y: event.y } : undefined;
		this.open = true;
	};

	public handleClose = () => {
		this.open = false;
	};

	public handleMakeThumbnail = async () => {
		this.open = false;

		if (!this.fileId || !this.treeId) {
			return;
		}

		const res = await changeTreeThumbnail(this.treeId, this.fileId);

		if (res.status >= 400) {
			showError(res.error?.description || `Error ${res.status} changing thumbnail.`);
		}
	};

	public handleDelete = async () => {
		this.open = false;

		if (!this.fileId) {
			return;
		}

		const res = await deleteFile(this.fileId);

		if (res.status >= 400) {
			showError(res.error?.description || `Error ${res.status} deleting file.`);
		}
	};

	public subscribe = () => {
		menuBus.on('showPhoto', this.handleShow);

		return () => {
			menuBus.off('showPhoto', this.handleShow);
		};
	};
}
