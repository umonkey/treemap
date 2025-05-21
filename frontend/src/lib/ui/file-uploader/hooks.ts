import { get, writable } from 'svelte/store';

export const load = () => {
	const input = writable<HTMLInputElement | null>(null);
	const files = writable<File[]>([]);

	const handleChange = (event: Event) => {
		const items = (event.target as HTMLInputElement).files;

		if (items && items.length > 0) {
			for (let i = 0; i < items.length; i++) {
				const file = items[i];
				files.update((currentFiles) => [...currentFiles, file]);
			}
		}

		console.debug(`[FileUploader] Have ${get(files).length} files selected.`);
	};

	return { input, files, handleChange };
};
