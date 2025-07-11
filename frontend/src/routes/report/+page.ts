import type { Load } from '@sveltejs/kit';

export const load: Load = ({
	url
}): {
	address: string | null;
} => {
	const address = url.searchParams.get('address');

	return {
		address
	};
};
