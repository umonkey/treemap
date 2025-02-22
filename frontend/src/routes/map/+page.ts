import type { Load } from '@sveltejs/kit';

export const load: Load = ({
	url
}): {
	searchQuery: string | null;
} => {
	const searchQuery = url.searchParams.get('q');

	return {
		searchQuery
	};
};
