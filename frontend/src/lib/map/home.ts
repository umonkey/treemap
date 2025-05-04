// Returns map page route.
// Depending on the search status, this does or does not add the search query.

import { routes } from '$lib/routes';
import { searchStore } from '$lib/stores';
import { get } from 'svelte/store';

export const mapHome = (): string => {
	const query = get(searchStore);

	if (query) {
		return routes.searchQuery(query);
	}

	return routes.map();
};
