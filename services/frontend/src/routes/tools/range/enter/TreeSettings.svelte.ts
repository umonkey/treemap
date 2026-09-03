import { rangeStore } from '../store.svelte';

export class TreeSettingsState {
	selectedTreeId = $derived(rangeStore.selectedTree);
	selectedTree = $derived(rangeStore.trees.find((t) => t.id === this.selectedTreeId));
	selectedIndex = $derived(rangeStore.trees.findIndex((t) => t.id === this.selectedTreeId));

	remove = () => {
		if (this.selectedTreeId) {
			rangeStore.removeTree(this.selectedTreeId);
		}
	};

	keep = () => {
		rangeStore.setSelectedTree(undefined);
	};
}
