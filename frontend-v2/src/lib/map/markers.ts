import type { Map } from 'leaflet';

export class Markers {
	private map;

	constructor(map: Map) {
		this.map = map;
		map.on('moveend', () => this.onMoveEnd());
	}

	private onMoveEnd() {
		console.log('MOVE END', this.map.getBounds());
	}
}
