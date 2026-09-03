import { rangeStore } from '../store.svelte';

export class TreeListLogic {
	trees = $derived(rangeStore.trees);

	removeTree = (id: string) => {
		rangeStore.removeTree(id);
	};
}
