// Loads data required to replace a tree, performs updates.
//
// TODO: fail if the previous tree is already "gone"?

import type { ITree, IReplaceTreeRequest } from '$lib/types';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import { apiClient } from '$lib/api';
import { get } from 'svelte/store';
import { goto, routes } from '$lib/routes';
import { locale } from '$lib/locale';
import { toast } from '@zerodevx/svelte-toast';
import { writable } from 'svelte/store';

export const editor = (tree_id: string) => {
	const loading = writable<boolean>(true);
	const busy = writable<boolean>(false);
	const loadError = writable<string | undefined>(undefined);
	const saveError = writable<string | undefined>(undefined);
	const tree = writable<ITree | undefined>(undefined);

	const uploading = writable<boolean>(false);
	const uploads = writable<string[]>([]);

	const species = writable<string>('');
	const height = writable<number>(0);
	const diameter = writable<number>(0);
	const circumference = writable<number>(0);
	const currentState = writable<string>('healthy');
	const year = writable<number>(new Date().getFullYear());
	const notes = writable<string>('');

	const reload = async (tree_id: string) => {
		console.debug(`[replace editor] Reloading tree ${tree_id}`);

		loading.set(true);
		loadError.set(undefined);

		await apiClient
			.getTree(tree_id)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					tree.set(res.data);

					if (res.data.replaced_by) {
						loadError.set('This tree has already been replaced.');
					}

					addUsers(res.data.users);
				} else if (res.error) {
					loadError.set(res.error.description);
				}
			})
			.finally(() => {
				loading.set(false);
			});
	};

	// Upload in progress, set the busy flag to prevent form submit.
	const handleUploading = (value: boolean) => {
		uploading.set(value);
	};

	// Some files finished uploading.
	const handleUploaded = (value: string[]) => {
		uploads.set(value);
	};

	const save = async () => {
		busy.set(true);
		saveError.set(undefined);

		const req = {
			species: get(species),
			height: get(height),
			circumference: get(circumference),
			diameter: get(diameter),
			state: get(currentState),
			year: get(year),
			notes: get(notes),
			files: get(uploads)
		} as IReplaceTreeRequest;

		await apiClient
			.replaceTree(tree_id, req)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					addTrees(res.data.trees);
					toast.push(locale.replaceSuccess());

					if (res.data.trees.length === 2) {
						goto(routes.treeDetails(res.data.trees[1].id));
					} else {
						goto(routes.treeDetails(tree_id));
					}
				} else if (res.error) {
					saveError.set(res.error.description);
				}
			})
			.finally(() => {
				busy.set(false);
			});
	};

	const close = () => {
		goto(routes.treeHistory(tree_id));
	};

	reload(tree_id);

	return {
		loading,
		busy,
		loadError,
		saveError,
		species,
		height,
		diameter,
		circumference,
		currentState,
		year,
		notes,
		reload,
		handleUploading,
		handleUploaded,
		uploading,
		save,
		close
	};
};
