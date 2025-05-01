// Loads data required to move the tree, performs updates.

import type { IChange, ITree, ILatLng } from '$lib/types';
import { apiClient } from '$lib/api';
import { get } from 'svelte/store';
import { goto, routes } from '$lib/routes';
import { locale } from '$lib/locale';
import { roundCoord } from '$lib/utils/strings';
import { toast } from '@zerodevx/svelte-toast';
import { writable } from 'svelte/store';

export const editor = (tree_id: string) => {
	const loading = writable<boolean>(true);
	const busy = writable<boolean>(false);
	const loadError = writable<string | undefined>(undefined);
	const saveError = writable<string | undefined>(undefined);
	const tree = writable<ITree | undefined>(undefined);
	const history = writable<IChange[]>([]);
	const value = writable<ILatLng>({ lat: 0, lng: 0 });
	const updated = writable<ILatLng>({ lat: 0, lng: 0 });

	const reload = async (tree_id: string) => {
		console.debug(`[location editor] Reloading tree ${tree_id}`);

		loading.set(true);
		loadError.set(undefined);

		const p1 = await apiClient.getTree(tree_id).then((res) => {
			if (res.status >= 200 && res.status < 300 && res.data) {
				tree.set(res.data);

				const ll = {
					lat: roundCoord(res.data.lat),
					lng: roundCoord(res.data.lon)
				};

				value.set(ll);
				updated.set(ll);
			} else if (res.error) {
				loadError.set(res.error.description);
			}
		});

		const p2 = await apiClient.getTreeHistory(tree_id).then((res) => {
			if (res.status >= 200 && res.status < 300 && res.data) {
				history.set(res.data.props);
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

		const ll = get(updated);

		await apiClient
			.updateTreeLocation(tree_id, ll.lat, ll.lng)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					toast.push(locale.measureLocationUpdated());
					goto(routes.treeDetails(tree_id));
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

	const handleChange = (v: ILatLng) => {
		updated.set(v);
	};

	reload(tree_id);

	return { loading, loadError, saveError, value, history, reload, save, close, handleChange };
};
