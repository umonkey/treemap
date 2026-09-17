import { mapBus } from '$lib/buses/mapBus';
import type { ILatLng } from '$lib/types';
import { LngLatBounds, type Map } from 'maplibre-gl';
import { mapState } from './MapLibre.svelte.ts';

export class MapEventBridgeLogic {
	private map: Map | undefined;

	private handleFit = ({ start, end }: { start: ILatLng; end: ILatLng }) => {
		mapState.hasMoved = true;

		if (this.map) {
			const bounds = new LngLatBounds();
			console.debug(`[map.events] Fit called.`);
			bounds.extend([start.lng, start.lat]);
			bounds.extend([end.lng, end.lat]);
			this.map.fitBounds(bounds, { padding: 50 });
		}
	};

	private handleMove = (ll: ILatLng) => {
		console.debug(`[map.events] Move to ${ll.lat},${ll.lng}`);

		mapState.hasMoved = true;
		this.map?.easeTo({ center: [ll.lng, ll.lat] });
	};

	private handleMoveOnce = (ll: ILatLng) => {
		if (mapState.hasMoved) {
			console.debug(`[map.events] Move (once) ignored to ${ll.lat},${ll.lng}`);
			return;
		}

		this.handleMove(ll);
	};

	public mount = (map: Map) => {
		this.map = map;
		mapBus.on('fit', this.handleFit);
		mapBus.on('move', this.handleMove);
		mapBus.on('moveOnce', this.handleMoveOnce);
	};

	public unmount = () => {
		mapBus.off('fit', this.handleFit);
		mapBus.off('move', this.handleMove);
		mapBus.off('moveOnce', this.handleMoveOnce);
		this.map = undefined;
	};
}
