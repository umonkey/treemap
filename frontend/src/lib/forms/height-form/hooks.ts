// Loads data required by the height editor, performs updates.

import type { IChange, ITree } from '$lib/types';
import { goto, routes } from '$lib/routes';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import { apiClient } from '$lib/api';
import { locale } from '$lib/locale';
import { toast } from '@zerodevx/svelte-toast';
import { writable } from 'svelte/store';
import { get } from 'svelte/store';

export const editor = (tree_id: string) => {
	const loading = writable<boolean>(true);
	const busy = writable<boolean>(false);
	const loadError = writable<string | undefined>(undefined);
	const saveError = writable<string | undefined>(undefined);
	const tree = writable<ITree | undefined>(undefined);
	const history = writable<IChange[]>([]);
	const value = writable<number>(0);

	const reload = async (tree_id: string) => {
		console.debug(`[height editor] Reloading tree ${tree_id}`);

		loading.set(true);
		loadError.set(undefined);

		const p1 = await apiClient.getTree(tree_id).then((res) => {
			if (res.status >= 200 && res.status < 300 && res.data) {
				tree.set(res.data);
				value.set(res.data.height ?? 0);
				addUsers(res.data.users);
			} else if (res.error) {
				loadError.set(res.error.description);
			}
		});

		const p2 = await apiClient.getTreeHistory(tree_id).then((res) => {
			if (res.status >= 200 && res.status < 300 && res.data) {
				history.set(res.data.props);
				addUsers(res.data.users);
			} else if (res.error) {
				loadError.set(res.error.description);
			}
		});

		await Promise.all([p1, p2]).finally(() => {
			loading.set(false);
		});
	};

	const save = async () => {
		busy.set(true);
		saveError.set(undefined);

		await apiClient
			.updateTreeHeight(tree_id, get(value))
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					addTrees([res.data]);
					toast.push(locale.measureHeightUpdated());
					goto(routes.mapPreview(tree_id));
				} else if (res.error) {
					saveError.set(res.error.description);
				}
			})
			.finally(() => {
				busy.set(false);
			});
	};

	const close = () => {
		goto(routes.mapPreview(tree_id));
	};

	const handleChange = (v: number) => {
		value.set(v);
	};

	reload(tree_id);

	return { loading, loadError, saveError, value, history, reload, save, close, handleChange };
};
