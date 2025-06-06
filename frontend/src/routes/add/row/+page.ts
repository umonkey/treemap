import { error, type Load } from '@sveltejs/kit';
import { type ILatLng } from '$lib/types';

const coord = (value: string | null): number => {
	if (!value) {
		return error(400);
	}

	return Number.parseFloat(value);
};

export const load: Load = ({
	url
}): {
	start: ILatLng;
	end: ILatLng;
} => {
	const start = {
		lat: coord(url.searchParams.get('alat')),
		lng: coord(url.searchParams.get('alng'))
	};

	const end = {
		lat: coord(url.searchParams.get('blat')),
		lng: coord(url.searchParams.get('blng'))
	};

	return {
		start,
		end
	};
};
