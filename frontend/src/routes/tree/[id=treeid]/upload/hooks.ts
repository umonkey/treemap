import { get, writable } from 'svelte/store';
import { apiClient } from '$lib/api';
import { routes, goto } from '$lib/routes';
import { toast } from '@zerodevx/svelte-toast';
import { locale } from '$lib/locale';

export const load = (treeId: string) => {
	// Set by the file picker when upload is in progress.
	const busy = writable<boolean>(false);

	// Contains ids of successfully uploaded files.
	const uploads = writable<string[]>([]);

	// Set when the form has files and can be submitted.
	const canSubmit = writable<boolean>(false);

	const handleBusy = (value: boolean) => {
		busy.set(value);
		canSubmit.set(!value && get(uploads).length > 0);
	};

	const handleChange = (value: string[]) => {
		uploads.set(value);
		canSubmit.set(value.length > 0 && !get(busy));
	};

	const handleSubmit = () => {
		busy.set(true);

		apiClient
			.addPhotos(treeId, get(uploads))
			.then((res) => {
				if (res.status === 202) {
					uploads.set([]);
					toast.push(locale.photosAdded());
					goto(routes.treeDetails(treeId));
				} else if (res.error) {
					console.error(`Error uploading photos: ${res.error.description}`);
				}
			})
			.finally(() => {
				busy.set(false);
			});
	};

	return { canSubmit, handleBusy, handleChange, handleSubmit };
};
