import { goto, routes } from '$lib/routes';
import { getMapillaryImage, type MapillaryImage } from '$lib/api/mapillary';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';
import { mapMarkerStore } from '$lib/stores/mapMarker.svelte';
import { mapBus } from '$lib/buses/mapBus';
import { LngLat } from 'maplibre-gl';

class PageState {
	id = $state<string>('');
	image = $state<MapillaryImage | null>(null);

	public handleClose = async () => {
		await goto(routes.home());
	};

	public handleMove = (angle: number) => {
		if (this.image) {
			mapRaysStore.rays = [
				{
					lat: this.image.lat,
					lng: this.image.lon,
					angle: (this.image.compass_angle + angle + 360) % 360
				}
			];
		}
	};

	public reload = async (id: string) => {
		if (this.id === id) {
			return;
		}

		this.id = id;
		const response = await getMapillaryImage(id);
		if (response.status >= 200 && response.status < 300 && response.data) {
			if (this.id !== id) {
				return;
			}
			this.image = response.data;

			const ll = { lat: this.image.lat, lng: this.image.lon };
			if (!mapMarkerStore.center) {
				mapMarkerStore.center = new LngLat(ll.lng, ll.lat);
			}
			mapBus.emit('map-once', ll);
		}
	};

	public cleanup = () => {
		this.id = '';
		this.image = null;
		mapMarkerStore.center = undefined;
		mapRaysStore.rays = [];
	};
}

export const pageState = new PageState();
