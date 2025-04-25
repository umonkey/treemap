// Loads data required by the state editor, performs updates.

import type { IChange, ITree } from '$lib/types';
import { goto, routes } from '$lib/routes';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import { apiClient, unwrap } from '$lib/api';
import { locale } from '$lib/locale';
import { toast } from '@zerodevx/svelte-toast';
import { writable } from 'svelte/store';
import { get } from 'svelte/store';

export const editor = (tree: ITree) => {
	const loading = writable<boolean>(true);
	const history = writable<IChange[]>([]);
	const value = writable<string>(tree.state ?? '');

	const reload = async (tree_id: string) => {
		console.debug(`[state editor] Reloading tree ${tree_id}`);

		loading.set(true);

		unwrap(await apiClient.getTreeHistory(tree_id))
			.then((res) => {
				history.set(res.props);
				addUsers(res.users);
			})
			.finally(() => {
				loading.set(false);
			});
	};

	const save = async () => {
		unwrap(await apiClient.updateTreeState(tree.id, get(value)))
			.then((res) => {
				addTrees([res]);
				toast.push(locale.measureStateUpdated());
				goto(routes.treeHistory(tree.id));
			})
			.catch(() => {
				toast.push('Error saving changes.');
			});
	};

	const close = () => {
		goto(routes.treeHistory(tree.id));
	};

	const handleChange = (e: Event) => {
		if (e.target) {
			value.set((e.target as HTMLInputElement).value);
		}
	};

	reload(tree.id);

	return { loading, value, history, reload, save, close, handleChange };
};
