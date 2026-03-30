import { searchStore } from '$lib/stores/searchStore';
import { get } from 'svelte/store';
import { goto, routes } from '$lib/routes';

export class SearchState {
	value = $state('');

	constructor() {
		this.value = get(searchStore) ?? '';

		searchStore.subscribe((storeValue) => {
			if (storeValue !== undefined && storeValue !== this.value) {
				this.value = storeValue;
			}
		});
	}

	commit = async () => {
		await goto(routes.searchQuery(this.value));
	};

	clear = () => {
		this.value = '';
		this.commit();
	};
}
