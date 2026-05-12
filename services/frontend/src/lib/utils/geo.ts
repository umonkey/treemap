import type { ILatLng } from '$lib/types';

/**
 * Calculates approximate distance in meters between two points.
 * Uses linear approximation which is accurate enough for short distances (e.g. < 1km).
 */
export const getDistance = (p1: ILatLng, p2: ILatLng): number => {
	// Average latitude for Yerevan is ~40.18
	const latMid = (p1.lat + p2.lat) / 2;
	const m_per_deg_lat = 111132;
	const m_per_deg_lng = 111320 * Math.cos((latMid * Math.PI) / 180);

	const dLat = (p1.lat - p2.lat) * m_per_deg_lat;
	const dLng = (p1.lng - p2.lng) * m_per_deg_lng;

	return Math.sqrt(dLat * dLat + dLng * dLng);
};
