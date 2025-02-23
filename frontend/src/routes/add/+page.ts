import type { Load } from '@sveltejs/kit';
import { DEFAULT_MAP_CENTER } from '$lib/constants';

const coord = (value: string | null, defaultValue: number): number => {
	if (!value) {
		return defaultValue;
	}

	return parseFloat(value);
};

export const load: Load = ({
	url
}): {
	lat: number;
	lng: number;
} => {
	const lat = coord(url.searchParams.get('lat'), DEFAULT_MAP_CENTER[0]);
	const lng = coord(url.searchParams.get('lng'), DEFAULT_MAP_CENTER[1]);

	return {
		lat,
		lng
	};
};
