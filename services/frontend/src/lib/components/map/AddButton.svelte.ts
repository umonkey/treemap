import { menuBus } from '$lib/buses/menuBus';
import type { Map } from 'maplibre-gl';

export class AddButtonLogic {
	public handleClick = (map: Map) => {
		const center = map.getCenter();
		menuBus.emit('showMap', { lat: center.lat, lng: center.lng });
	};
}
