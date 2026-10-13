import { queuePhoto } from '$lib/upload';

export class GalleryUploader {
	handleChange = async (treeId: string, event: Event) => {
		const target = event.target as HTMLInputElement;
		const files = target.files;

		if (files && files.length > 0) {
			for (const file of Array.from(files)) {
				await queuePhoto(treeId, file);
			}

			target.value = ''; // Reset to allow selecting the same file again.
		}
	};
}
