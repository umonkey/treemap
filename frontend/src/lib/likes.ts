import { apiClient } from '$lib/api';
import { likeStore, haveOwnLikes } from '$lib/stores/likeStore';
import { get } from 'svelte/store';
import type { ILike } from '$lib/types';

export const preloadMeLikes = async () => {
	if (get(haveOwnLikes)) {
		console.debug('[likes] Already have own likes.');
		return;
	}

	console.debug('[likes] Preloading own likes.');

	const { status, data } = await apiClient.getMeLikes();

	if (status === 200 && data) {
		likeStore.update(() => data.likes.map((x: ILike) => x.tree_id));
	}
};

export const like = async (tree: string) => {
	const res = await apiClient.likeTree(tree);

	if (res.status < 400) {
		likeStore.update((likes: string[] | undefined) => {
			const updated = likes ? [...(likes ?? [])].filter((t) => t !== tree) : [];
			updated.push(tree);
			return updated;
		});

		console.debug(`[likes] Tree ${tree} liked.`);
	}
};

export const unlike = async (tree: string) => {
	const res = await apiClient.unlikeTree(tree);

	if (res.status < 400) {
		likeStore.update((likes: string[] | undefined) => {
			const updated = [...(likes ?? [])].filter((t) => t !== tree);
			return updated;
		});

		console.debug(`[likes] Tree ${tree} unliked.`);
	}
};
