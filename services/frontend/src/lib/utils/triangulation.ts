import type { ILatLng } from '$lib/types';

export interface IGcpInput {
	lat: number;
	lng: number;
	radius: number; // in meters
}

export function triangulateTree(
	gcps: IGcpInput[],
	operatorPosition?: { lat: number; lng: number } | null
): ILatLng | null {
	const validGcps = gcps.filter(
		(g) =>
			g &&
			!Number.isNaN(g.lat) &&
			!Number.isNaN(g.lng) &&
			!(g.lat === 0 && g.lng === 0) &&
			g.radius > 0
	);
	if (validGcps.length === 0) return null;
	if (validGcps.length === 1) {
		return { lat: validGcps[0].lat, lng: validGcps[0].lng };
	}

	const origin = validGcps[0];
	const mPerLat = 111320;
	const mPerLng = 111320 * Math.cos((origin.lat * Math.PI) / 180);

	const points = validGcps.map((g) => ({
		x: (g.lng - origin.lng) * mPerLng,
		y: (g.lat - origin.lat) * mPerLat,
		radius: g.radius
	}));

	const candidates: Array<{ x: number; y: number }> = [];

	for (let i = 0; i < points.length; i++) {
		for (let j = i + 1; j < points.length; j++) {
			const c1 = points[i];
			const c2 = points[j];
			const dx = c2.x - c1.x;
			const dy = c2.y - c1.y;
			const d = Math.sqrt(dx * dx + dy * dy);

			if (d === 0) continue;
			if (d > c1.radius + c2.radius) {
				const t = c1.radius / (c1.radius + c2.radius);
				candidates.push({
					x: c1.x + t * dx,
					y: c1.y + t * dy
				});
				continue;
			}
			if (d < Math.abs(c1.radius - c2.radius)) {
				candidates.push({
					x: (c1.x + c2.x) / 2,
					y: (c1.y + c2.y) / 2
				});
				continue;
			}

			const a = (c1.radius * c1.radius - c2.radius * c2.radius + d * d) / (2 * d);
			const hSq = c1.radius * c1.radius - a * a;
			const h = hSq > 0 ? Math.sqrt(hSq) : 0;

			const x2 = c1.x + (a * dx) / d;
			const y2 = c1.y + (a * dy) / d;

			const rx = (-dy * h) / d;
			const ry = (dx * h) / d;

			candidates.push({ x: x2 + rx, y: y2 + ry });
			if (h > 1e-6) {
				candidates.push({ x: x2 - rx, y: y2 - ry });
			}
		}
	}

	if (candidates.length === 0) {
		const avgX = points.reduce((sum, p) => sum + p.x, 0) / points.length;
		const avgY = points.reduce((sum, p) => sum + p.y, 0) / points.length;
		candidates.push({ x: avgX, y: avgY });
	}

	let bestCandidate = candidates[0];
	let minScore = Number.POSITIVE_INFINITY;

	for (const cand of candidates) {
		let score = 0;
		for (const p of points) {
			const dist = Math.sqrt((cand.x - p.x) ** 2 + (cand.y - p.y) ** 2);
			score += Math.abs(dist - p.radius);
		}
		if (
			operatorPosition &&
			!Number.isNaN(operatorPosition.lat) &&
			!Number.isNaN(operatorPosition.lng)
		) {
			const opX = (operatorPosition.lng - origin.lng) * mPerLng;
			const opY = (operatorPosition.lat - origin.lat) * mPerLat;
			const distToOp = Math.sqrt((cand.x - opX) ** 2 + (cand.y - opY) ** 2);
			score += distToOp * 0.001;
		}
		if (score < minScore) {
			minScore = score;
			bestCandidate = cand;
		}
	}

	const lng = origin.lng + bestCandidate.x / mPerLng;
	const lat = origin.lat + bestCandidate.y / mPerLat;

	return { lat, lng };
}
