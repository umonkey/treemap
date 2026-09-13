import { menuBus } from '$lib/buses/menuBus';
import type { Map } from 'maplibre-gl';

export class AddButtonLogic {
	public handleClick = (map: Map, button: HTMLElement) => {
		const center = map.getCenter();
		const rect = button.getBoundingClientRect();

		menuBus.emit('showMap', {
			lat: center.lat,
			lng: center.lng,
			x: rect.left,
			y: rect.top
		});
	};
}
