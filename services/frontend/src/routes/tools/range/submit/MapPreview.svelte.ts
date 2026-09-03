import { Map, LngLatBounds } from 'maplibre-gl';
import { config } from '$lib/env';
import { locale } from '$lib/locale';
import type { ITriangulatedTree } from '../store.svelte';

export class SubmitMapPreviewState {
	map = $state.raw<Map | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/base-v4-light/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	getMapCenter = (trees: ITriangulatedTree[]): [number, number] => {
		const first = trees[0];
		if (
			first &&
			!Number.isNaN(first.lat) &&
			!Number.isNaN(first.lng) &&
			!(first.lat === 0 && first.lng === 0)
		) {
			return [first.lng, first.lat];
		}
		return [44.5152, 40.1872];
	};

	fitBounds = (trees: ITriangulatedTree[]): void => {
		if (!this.map) return;
		const bounds = new LngLatBounds();
		let hasPoints = false;

		for (const t of trees) {
			if (t && !Number.isNaN(t.lat) && !Number.isNaN(t.lng) && !(t.lat === 0 && t.lng === 0)) {
				bounds.extend([t.lng, t.lat]);
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
	};
}
