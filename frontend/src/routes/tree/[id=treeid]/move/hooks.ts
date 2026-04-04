// Loads data required to move the tree, performs updates.

import { getTree, updateTreeLocation } from '$lib/api/trees';
import { goto, routes } from '$lib/routes';
import type { ILatLng, ITree } from '$lib/types';
import { roundCoord } from '$lib/utils/strings';
import { get } from 'svelte/store';
import { writable } from 'svelte/store';

export const editor = (tree_id: string) => {
	const loading = writable<boolean>(true);
	const busy = writable<boolean>(false);
	const loadError = writable<string | undefined>(undefined);
	const saveError = writable<string | undefined>(undefined);
	const tree = writable<ITree | undefined>(undefined);
	const value = writable<ILatLng>({ lat: 0, lng: 0 });
	const updated = writable<ILatLng>({ lat: 0, lng: 0 });

	const reload = async (tree_id: string) => {
		console.debug(`[location editor] Reloading tree ${tree_id}`);

		loading.set(true);
		loadError.set(undefined);

		const p1 = await getTree(tree_id).then((res) => {
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

		await Promise.all([p1]).finally(() => {
			loading.set(false);
		});
	};

	const save = async () => {
		busy.set(true);
		saveError.set(undefined);

		const ll = get(updated);

		await updateTreeLocation(tree_id, ll.lat, ll.lng)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
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

	const handleChange = (v: ILatLng) => {
		updated.set(v);
	};

	reload(tree_id);

	return {
		loading,
		busy,
		loadError,
		saveError,
		tree,
		value,
		reload,
		save,
		close,
		handleChange
	};
};
