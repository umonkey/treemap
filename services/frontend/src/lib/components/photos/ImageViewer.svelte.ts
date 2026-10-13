export class ImageViewerLogic {
	onClose: () => void = () => {};

	bind = (onClose: () => void) => {
		this.onClose = onClose;
	};

	handleKeydown = (event: KeyboardEvent) => {
		if (event.key === 'Escape') this.onClose();
	};
}
