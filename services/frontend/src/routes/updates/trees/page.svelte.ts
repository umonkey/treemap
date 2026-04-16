import { getUpdatedTrees } from '$lib/api/trees';
import { routes } from '$lib/routes';
import { addTrees } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type { ITree } from '$lib/types';
import { formatDate } from '$lib/utils/strings';
import { formatSpecies } from '$lib/utils/trees';

const PAGE_SIZE = 24;

export type Tile = {
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

class PageState {
	tiles = $state<Tile[]>([]);
	loading = $state<boolean>(true);
	error = $state<boolean>(false);
	skip = $state<number>(0);

	reload = async () => {
		const params = {
			count: PAGE_SIZE,
			skip: this.skip
		};

		try {
			const { status, data } = await getUpdatedTrees(params);
			if (status < 400 && data) {
				addTrees(data.trees);
				addUsers(data.users);

				const added = data.trees.map(formatTile);
				this.tiles = [...this.tiles, ...added];
				this.error = false;
			} else {
				this.error = true;
			}
		} catch (err) {
			console.error('Failed to load updated trees:', err);
			this.error = true;
		} finally {
			this.loading = false;
		}
	};

	handleLoadMore = () => {
		this.skip += PAGE_SIZE;
		this.reload();
	};
}

export const pageState = new PageState();
