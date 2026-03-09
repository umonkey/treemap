import { get, writable } from 'svelte/store';
import { apiClient } from '$lib/api';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type { ITree } from '$lib/types';
import { formatDate } from '$lib/utils/strings';
import { formatSpecies } from '$lib/utils/trees';
import { routes } from '$lib/routes';

const PAGE_SIZE = 24;

type Tile = {
	id: string;
	link: string;
	species: string;
	address: string | null;
	image: string | null;
	updated_at: string;
};

const formatTile = (tree: ITree): Tile => {
	return {
		id: tree.id,
		link: routes.treeDetails(tree.id),
		species: formatSpecies(tree.species),
		address: tree.address,
		image: tree.thumbnail_id ? routes.file(tree.thumbnail_id) : null,
		updated_at: formatDate(tree.added_at)
	};
};

export const hooks = () => {
	const tiles = writable<Tile[]>([]);
	const loading = writable<boolean>(true);
	const error = writable<boolean>(false);
	const skip = writable<number>(0);

	const reload = () => {
		const params = {
			count: PAGE_SIZE,
			skip: get(skip)
		};

		apiClient
			.getUpdatedTrees(params)
			.then(({ status, data }) => {
				if (status < 400 && data) {
					addTrees(data.trees);
					addUsers(data.users);

					const added = data.trees.map(formatTile);
					tiles.update((items) => [...items, ...added]);
				} else {
					error.set(true);
				}
			})
			.finally(() => {
				loading.set(false);
			});
	};

	const handleLoadMore = () => {
		skip.update((value) => value + PAGE_SIZE);
		reload();
	};

	reload();

	return { loading, error, tiles, handleLoadMore };
};
