import { hasPermission } from '$lib/stores/authStore';
import { queuePhoto } from '$lib/upload';
import { get } from 'svelte/store';

export class ClipboardUploader {
	handlePaste = async (treeId: string, event: ClipboardEvent) => {
		if (!get(hasPermission)('tree:edit')) {
			return;
		}

		const items = event.clipboardData?.items;
		if (!items) {
			return;
		}

		const files: File[] = [];
		for (const item of Array.from(items)) {
			if (item.kind === 'file' && item.type.startsWith('image/')) {
				const file = item.getAsFile();
				if (file) {
					files.push(file);
				}
			}
		}

		if (files.length === 0) {
			return; // Let the browser handle text/HTML pastes.
		}

		event.preventDefault();

		for (const file of files) {
			await queuePhoto(treeId, file);
		}
	};
}
