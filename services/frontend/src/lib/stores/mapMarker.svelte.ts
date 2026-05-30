import type { LngLat } from 'maplibre-gl';

class MapMarkerStore {
	center = $state<LngLat | undefined>();
}

export const mapMarkerStore = new MapMarkerStore();
