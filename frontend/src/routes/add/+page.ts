import type { Load } from '@sveltejs/kit';
import { DEFAULT_MAP_CENTER } from '$lib/constants';

export const load: Load = ({
	url
}): Promise<{
	lat: number;
	lng: number;
}> => {
	const lat = parseFloat(url.searchParams.get('lat') ?? DEFAULT_MAP_CENTER[0]);
	const lng = parseFloat(url.searchParams.get('lng') ?? DEFAULT_MAP_CENTER[1]);

	return {
		lat,
		lng
	};
};
