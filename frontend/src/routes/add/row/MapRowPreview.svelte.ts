import { mapBus } from '$lib/buses';
import { spreadDots } from '$lib/map';
import type { ILatLng } from '$lib/types';

const renderGeoJSON = (start: ILatLng, end: ILatLng, count: number) => {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const res: any = {
		type: 'FeatureCollection',
		features: []
	};

	res.features.push({
		type: 'Feature',
		geometry: {
			type: 'LineString',
			coordinates: [
				[start.lng, start.lat],
				[end.lng, end.lat]
			]
		}
	});

	for (const dot of spreadDots(start, end, count)) {
		res.features.push({
			type: 'Feature',
			geometry: {
				type: 'Point',
				coordinates: [dot.lng, dot.lat]
			}
		});
	}

	return res;
};

class PreviewState {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	data = $state.raw<any>();

	public update = (start: ILatLng, end: ILatLng, count: number) => {
		mapBus.emit('fit', { start, end });
		this.data = renderGeoJSON(start, end, count);
	};
}

export const previewState = new PreviewState();
