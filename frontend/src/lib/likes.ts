import { apiClient } from '$lib/api';
import { likeStore, haveOwnLikes } from '$lib/stores/likeStore';
import { get } from 'svelte/store';

export const preloadMeLikes = async () => {
	if (get(haveOwnLikes)) {
		console.debug('[likes] Already have own likes.');
		return;
	}

	console.debug('[likes] Preloading own likes.');
	const res = await apiClient.getMeLikes();

	if (res.status === 200) {
		likeStore.update(() => res.data.likes.map((x) => x.tree_id));
	}
};

export const like = async (tree: string) => {
	const res = await apiClient.likeTree(tree);

	if (res.status < 400) {
		likeStore.update((likes) => {
			const updated = likes ? [...likes].filter((t) => t !== tree) : [];
			updated.push(tree);
			return updated;
		});

		console.debug(`[likes] Tree ${tree} liked.`);
	}
};

export const unlike = async (tree: string) => {
	const res = await apiClient.unlikeTree(tree);

	if (res.status < 400) {
		likeStore.update((likes) => {
			const updated = [...likes].filter((t) => t !== tree);
			return updated;
		});

		console.debug(`[likes] Tree ${tree} unliked.`);
	}
};
