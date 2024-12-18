import type { Load } from '@sveltejs/kit';

export const load: Load = ({
	url
}): Promise<{
	lat: number;
	lng: number;
}> => {
	const lat = parseFloat(url.searchParams.get('lat'));
	const lng = parseFloat(url.searchParams.get('lng'));

	return {
		lat,
		lng
	};
};
