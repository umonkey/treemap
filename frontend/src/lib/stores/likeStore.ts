import { derived, writable } from "svelte/store";

export const likeStore = writable<string[] | undefined>(undefined);

export const isLiked = derived(likeStore, ($likeStore) => {
	return (id: string): boolean => ($likeStore || []).some((x) => x === id);
});

export const haveOwnLikes = derived(
	likeStore,
	($likeStore) => $likeStore !== undefined,
);
