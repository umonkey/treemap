import { Map, LngLat } from 'maplibre-gl';
import { config } from '$lib/env';
import { locale } from '$lib/locale';
import { locationStore } from '$lib/stores/locationStore';
import { get } from 'svelte/store';
import type { ILatLng } from '$lib/types';

export class LocationPickerState {
	map = $state.raw<Map | undefined>(undefined);
	selectedLocation = $state<ILatLng | null>(null);

	layer = `https://api.maptiler.com/maps/openstreetmap/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	handleMapClick = (e: { lngLat: LngLat }) => {
		this.selectedLocation = { lat: e.lngLat.lat, lng: e.lngLat.lng };
	};

	useMyLocation = () => {
		const pos = get(locationStore);
		if (pos) {
			this.selectedLocation = { lat: pos.lat, lng: pos.lng };
			if (this.map) {
				this.map.flyTo({ center: [pos.lng, pos.lat], zoom: 16 });
			}
		} else if ('geolocation' in navigator) {
			navigator.geolocation.getCurrentPosition(
				(position) => {
					const loc = { lat: position.coords.latitude, lng: position.coords.longitude };
					this.selectedLocation = loc;
					if (this.map) {
						this.map.flyTo({ center: [loc.lng, loc.lat], zoom: 16 });
					}
				},
				() => {},
				{ enableHighAccuracy: true }
			);
		}
	};

	fitInitial = (initial?: ILatLng | null) => {
		if (!this.map) return;
		const center = initial || get(locationStore) || { lat: 40.1872, lng: 44.5152 };
		this.map.setCenter([center.lng, center.lat]);
		this.map.setZoom(15);
	};
}
