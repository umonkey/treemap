import type { ILatLng } from '$lib/types';

export class PointsLayerLogic {
	isValidPoint = (point: ILatLng | null | undefined): point is ILatLng => {
		return (
			point != null &&
			!Number.isNaN(point.lat) &&
			!Number.isNaN(point.lng) &&
			!(point.lat === 0 && point.lng === 0)
		);
	};
}
