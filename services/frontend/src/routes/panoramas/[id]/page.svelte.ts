import { goto, routes } from '$lib/routes';
import { getMapillaryImage, type MapillaryImage } from '$lib/api/mapillary';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';

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
		this.id = id;
		const response = await getMapillaryImage(id);
		if (response.status >= 200 && response.status < 300 && response.data) {
			this.image = response.data;
		}
	};
}

export const pageState = new PageState();
