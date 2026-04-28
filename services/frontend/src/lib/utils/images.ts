/**
 * Create a thumbnail from a Blob or File.
 */
export async function createThumbnail(blob: Blob, maxSize = 200): Promise<Blob> {
	let image: ImageBitmap | HTMLImageElement;

	try {
		image = await createImageBitmap(blob);
	} catch (e) {
		console.warn('createImageBitmap failed, falling back to HTMLImageElement:', e);
		image = await new Promise<HTMLImageElement>((resolve, reject) => {
			const img = new Image();
			const url = URL.createObjectURL(blob);
			img.onload = () => {
				URL.revokeObjectURL(url);
				resolve(img);
			};
			img.onerror = () => {
				URL.revokeObjectURL(url);
				reject(new Error('Failed to load image for thumbnail fallback'));
			};
			img.src = url;
		});
	}

	const { width, height } =
		image instanceof ImageBitmap
			? image
			: { width: image.naturalWidth, height: image.naturalHeight };

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
		if (image instanceof ImageBitmap) image.close();
		throw new Error('Failed to get canvas context');
	}

	ctx.drawImage(image, 0, 0, newWidth, newHeight);

	if (image instanceof ImageBitmap) {
		image.close();
	}

	const result = await (canvas instanceof OffscreenCanvas
		? canvas.convertToBlob({ type: 'image/jpeg', quality: 0.8 })
		: new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, 'image/jpeg', 0.8)));

	if (!result) {
		throw new Error('Failed to create thumbnail blob');
	}

	return result;
}
