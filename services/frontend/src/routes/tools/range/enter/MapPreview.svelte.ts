import { Map, LngLatBounds } from 'maplibre-gl';
import { config } from '$lib/env';
import { locale } from '$lib/locale';
import type { ILatLng } from '$lib/types';
import circle from '@turf/circle';
import type { ITriangulatedTree } from '../store.svelte';

export interface IGcpWithRadius extends ILatLng {
	radius: number;
	index: number;
	label: string;
}

export class RangeMapPreviewState {
	map = $state.raw<Map | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	fitBounds(
		gcps: IGcpWithRadius[],
		suggestedLocation?: ILatLng | null,
		operatorPosition?: ILatLng | null,
		trees?: ITriangulatedTree[]
	): void {
		if (!this.map) return;
		const bounds = new LngLatBounds();
		let hasPoints = false;

		for (const g of gcps) {
			if (!Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)) {
				bounds.extend([g.lng, g.lat]);
				hasPoints = true;
				if (g.radius > 0) {
					const c = circle([g.lng, g.lat], g.radius, { units: 'meters' });
					if (c.geometry?.coordinates?.[0]) {
						for (const ring of c.geometry.coordinates[0]) {
							bounds.extend(ring as [number, number]);
						}
					}
				}
			}
		}

		if (
			suggestedLocation &&
			!Number.isNaN(suggestedLocation.lat) &&
			!Number.isNaN(suggestedLocation.lng)
		) {
			bounds.extend([suggestedLocation.lng, suggestedLocation.lat]);
			hasPoints = true;
		}

		if (
			operatorPosition &&
			!Number.isNaN(operatorPosition.lat) &&
			!Number.isNaN(operatorPosition.lng)
		) {
			bounds.extend([operatorPosition.lng, operatorPosition.lat]);
			hasPoints = true;
		}

		if (trees && Array.isArray(trees)) {
			for (const t of trees) {
				if (t && !Number.isNaN(t.lat) && !Number.isNaN(t.lng)) {
					bounds.extend([t.lng, t.lat]);
					hasPoints = true;
				}
			}
		}

		if (hasPoints) {
			requestAnimationFrame(() => {
				if (this.map) {
					this.map.fitBounds(bounds, { padding: 40, animate: false });
				}
			});
		}
	}
}
