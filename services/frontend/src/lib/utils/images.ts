/**
 * Create a thumbnail from a Blob or File.
 */
export async function createThumbnail(blob: Blob, maxSize = 200): Promise<Blob> {
	const imageBitmap = await createImageBitmap(blob);
	const { width, height } = imageBitmap;

	let newWidth: number;
	let newHeight: number;

	if (width > height) {
		newWidth = maxSize;
		newHeight = (height / width) * maxSize;
	} else {
		newHeight = maxSize;
		newWidth = (width / height) * maxSize;
	}

	const canvas =
		typeof OffscreenCanvas !== 'undefined'
			? new OffscreenCanvas(newWidth, newHeight)
			: document.createElement('canvas');

	if (canvas instanceof HTMLCanvasElement) {
		canvas.width = newWidth;
		canvas.height = newHeight;
	}

	const ctx = canvas.getContext('2d') as
		| OffscreenCanvasRenderingContext2D
		| CanvasRenderingContext2D;

	if (!ctx) {
		throw new Error('Failed to get canvas context');
	}

	ctx.drawImage(imageBitmap, 0, 0, newWidth, newHeight);
	imageBitmap.close();

	const result = await (canvas instanceof OffscreenCanvas
		? canvas.convertToBlob({ type: 'image/jpeg', quality: 0.8 })
		: new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, 'image/jpeg', 0.8)));

	if (!result) {
		throw new Error('Failed to create thumbnail blob');
	}

	return result;
}
