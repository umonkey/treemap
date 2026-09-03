import { Map, LngLatBounds } from 'maplibre-gl';
import { config } from '$lib/env';
import { locale } from '$lib/locale';
import type { ILatLng } from '$lib/types';
import circle from '@turf/circle';
import type { ITriangulatedTree } from '../store.svelte';
import { locationStore } from '$lib/stores/locationStore';
import { get } from 'svelte/store';

export interface IGcpWithRadius extends ILatLng {
	radius: number;
	index: number;
	label: string;
}

export class RangeMapPreviewState {
	map = $state.raw<Map | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/base-v4-light/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	get operatorPos() {
		return get(locationStore);
	}

	getValidGcp = (gcps: IGcpWithRadius[]) =>
		gcps.find(
			(g) => g && !Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)
		);

	getMapCenter = (gcps: IGcpWithRadius[]): [number, number] => {
		const validGcp = this.getValidGcp(gcps);
		return validGcp
			? ([validGcp.lng, validGcp.lat] as [number, number])
			: ([44.5152, 40.1872] as [number, number]);
	};

	fitBounds = (
		gcps: IGcpWithRadius[],
		trees?: ITriangulatedTree[],
		suggestedLocation?: ILatLng | null
	): void => {
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
	};
}
