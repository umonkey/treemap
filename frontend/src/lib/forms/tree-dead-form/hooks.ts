// Loads data required by the state editor, performs updates.

import type { IChange, ITree } from '$lib/types';
import { goto, routes } from '$lib/routes';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import { apiClient } from '$lib/api';
import { locale } from '$lib/locale';
import { toast } from '@zerodevx/svelte-toast';
import { writable } from 'svelte/store';

export const editor = (tree_id: string, state: string) => {
	const loading = writable<boolean>(true);
	const error = writable<string | undefined>(undefined);
	const busy = writable<boolean>(false);
	const tree = writable<ITree | undefined>(undefined);
	const history = writable<IChange[]>([]);

	const reload = async (tree_id: string) => {
		console.debug(`[height editor] Reloading tree ${tree_id}`);

		loading.set(true);
		error.set(undefined);

		const p1 = apiClient.getTree(tree_id).then((res) => {
			if (res.status === 200 && res.data) {
				tree.set(res.data);
				addTrees([res.data]);
				addUsers(res.data.users);
			} else if (res.error) {
				error.set(res.error.description);
			}
		});

		const p2 = apiClient.getTreeHistory(tree_id).then((res) => {
			if (res.status === 200 && res.data) {
				history.set(res.data.props);
				addUsers(res.data.users);
			} else if (res.error) {
				error.set(res.error.description);
			}
		});

		await Promise.all([p1, p2]).finally(() => {
			loading.set(false);
		});
	};

	const save = async () => {
		busy.set(true);

		await apiClient
			.updateTreeState(tree_id, state)
			.then((res) => {
				if (res.status === 200 && res.data) {
					addTrees([res.data]);
					toast.push(locale.measureHeightUpdated());
					goto(routes.mapPreview(tree_id));
				} else if (res.error) {
					error.set(res.error.description);
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

	return { loading, busy, tree, history, reload, save, close };
};
