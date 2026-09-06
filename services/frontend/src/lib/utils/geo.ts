export { getDistance } from './index';

const M_PER_LAT = 111320;
const R_EARTH = 6371000; // meters

export function getOffsetDelta(lat: number, meters = 0.1) {
	const deltaLat = meters / M_PER_LAT;
	const cosLat = Math.cos((lat * Math.PI) / 180);
	const mPerLng = M_PER_LAT * (Math.abs(cosLat) > 1e-6 ? cosLat : 1e-6);
	const deltaLon = meters / mPerLng;
	return { deltaLat, deltaLon };
}

export function calculateDestination(
	lat: number,
	lng: number,
	angle: number,
	distanceMeters: number
): [number, number] {
	const brng = (angle * Math.PI) / 180;
	const lat1 = (lat * Math.PI) / 180;
	const lon1 = (lng * Math.PI) / 180;

	const lat2 = Math.asin(
		Math.sin(lat1) * Math.cos(distanceMeters / R_EARTH) +
			Math.cos(lat1) * Math.sin(distanceMeters / R_EARTH) * Math.cos(brng)
	);
	const lon2 =
		lon1 +
		Math.atan2(
			Math.sin(brng) * Math.sin(distanceMeters / R_EARTH) * Math.cos(lat1),
			Math.cos(distanceMeters / R_EARTH) - Math.sin(lat1) * Math.sin(lat2)
		);

	return [(lon2 * 180) / Math.PI, (lat2 * 180) / Math.PI];
}

export function roundOffset(value: number, decimals = 7): number {
	if (typeof value !== 'number' || Number.isNaN(value)) {
		return 0;
	}
	return Number(value.toFixed(decimals));
}
