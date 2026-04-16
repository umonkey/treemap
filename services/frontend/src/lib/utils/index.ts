import { type ILatLng } from '$lib/types';
export { roundCoord } from './strings';

export const getDistance = (p1: ILatLng, p2: ILatLng): number => {
	// Calculate the distance between two points in meters.
	const R = 6371000; // Radius of the Earth in meters
	const φ1 = (p1.lat * Math.PI) / 180; // φ, λ in radians
	const φ2 = (p2.lat * Math.PI) / 180;
	const Δφ = ((p2.lat - p1.lat) * Math.PI) / 180;
	const Δλ = ((p2.lng - p1.lng) * Math.PI) / 180;

	const a =
		Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
		Math.cos(φ1) * Math.cos(φ2) * Math.sin(Δλ / 2) * Math.sin(Δλ / 2);
	const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

	return Math.round(R * c * 100) / 100; // Distance in meters
};
