import { goto, routes } from '$lib/routes';
import { getPanoramasImage, type PanoramaImage } from '$lib/api/panoramas';
import { mapRaysStore } from '$lib/stores/mapRays.svelte';
import { mapBus } from '$lib/buses/mapBus';

class PageState {
	id = $state<string>('');
	image = $state<PanoramaImage | null>(null);
	angle = $state<number>(0);

	public handleClose = async () => {
		await goto(routes.home());
	};

	public handleMove = (angle: number) => {
		this.angle = angle;
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
		if (this.id === id) return;
		this.id = id;
		this.image = null;

		const imageRes = await getPanoramasImage(id);

		if (imageRes.data) {
			this.image = imageRes.data;

			const ll = { lat: this.image.lat, lng: this.image.lon };
			mapBus.emit('pin', ll);
			mapBus.emit('map-once', ll);
		}
	};

	public cleanup = () => {
		this.id = '';
		this.image = null;
		mapBus.emit('pin', undefined);
		mapRaysStore.rays = [];
	};
}

export const pageState = new PageState();
