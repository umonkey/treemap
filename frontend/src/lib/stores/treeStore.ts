import type { ITree } from '$lib/types';
import { derived, writable } from 'svelte/store';

type TreeMap = {
	[key: string]: ITree;
};

export const treeStore = writable<TreeMap>({});

export const getTree = derived(treeStore, ($treeStore) => {
	return (id: string) => $treeStore[id] ?? undefined;
});

export const addTrees = (trees: ITree[]) => {
	treeStore.update((store) => {
		for (const tree of trees) {
			store[tree.id] = tree;
		}

		return store;
	});
};
