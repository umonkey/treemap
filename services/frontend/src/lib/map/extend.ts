import type { IBounds } from '$lib/types';

export const extendBounds = ({ n, e, s, w }: IBounds, factor = 0.5): IBounds => {
	const dLat = n - s;
	const dLon = e - w;

	return {
		n: n + dLat * factor,
		e: e + dLon * factor,
		s: s - dLat * factor,
		w: w - dLon * factor
	};
};
