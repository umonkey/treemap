import type { ILatLng } from '$lib/types';
import { mapBus } from '$lib/buses';
import { spreadDots } from '$lib/map';

const renderGeoJSON = (start: ILatLng, end: ILatLng, count: number) => {
	const res = {
		type: 'FeatureCollection',
		features: [],
	};

	res.features.push({
		type: 'Feature',
		geometry: {
			type: 'LineString',
			coordinates: [
				[start.lng, start.lat],
				[end.lng, end.lat],
			],
		}
	});

	for (const dot of spreadDots(start, end, count)) {
		res.features.push({
			type: 'Feature',
			geometry: {
				type: 'Point',
				coordinates: [dot.lng, dot.lat],
			}
		});
	}

	return res;
}

class PreviewState {
	data = $state.raw();

	public update = (start: ILatLng, end: ILatLng, count: number) => {
		mapBus.emit('fit', { start, end });
		this.data = renderGeoJSON(start, end, count);
	}
}

export const previewState = new PreviewState();
