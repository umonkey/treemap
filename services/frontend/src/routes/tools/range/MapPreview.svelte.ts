import { Map, LngLatBounds } from 'maplibre-gl';
import { config } from '$lib/env';
import { locale } from '$lib/locale';
import type { ILatLng } from '$lib/types';

export interface IGcpWithRadius extends ILatLng {
	radius: number;
	index: number;
	label: string;
}

export class RangeMapPreviewState {
	map = $state.raw<Map | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/base-v4-light/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	fitBounds(gcps: IGcpWithRadius[]): void {
		if (!this.map) return;
		const bounds = new LngLatBounds();
		let hasPoints = false;

		for (const g of gcps) {
			if (g && !Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)) {
				bounds.extend([g.lng, g.lat]);
				hasPoints = true;
			}
		}

		if (hasPoints) {
			requestAnimationFrame(() => {
				if (this.map) {
					this.map.fitBounds(bounds, { padding: 40, maxZoom: 16, animate: false });
				}
			});
		}
	}
}
