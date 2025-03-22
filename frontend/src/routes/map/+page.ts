import type { Load } from '@sveltejs/kit';

export const load: Load = ({
	url
}): {
	searchQuery: string | null;
	preview: string | null;
} => {
	const searchQuery = url.searchParams.get('q');
	const preview = url.searchParams.get('preview');

	return {
		searchQuery,
		preview
	};
};
