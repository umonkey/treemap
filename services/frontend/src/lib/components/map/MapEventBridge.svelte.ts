import { mapBus } from '$lib/buses/mapBus';
import type { ILatLng } from '$lib/types';
import { LngLatBounds, type Map } from 'maplibre-gl';

export class MapEventBridgeLogic {
	private map: Map | undefined;
	private moving = false;
	private hasMoved = false;

	private handleMoveStart = () => {
		this.moving = true;
	};

	private handleMoveEnd = () => {
		if (this.moving) {
			this.hasMoved = true;
		}

		this.moving = false;
	};

	private handleFit = ({ start, end }: { start: ILatLng; end: ILatLng }) => {
		if (this.map) {
			const bounds = new LngLatBounds();
			console.debug(`[map.events] Fit called.`);
			bounds.extend([start.lng, start.lat]);
			bounds.extend([end.lng, end.lat]);
			this.map.fitBounds(bounds, { padding: 50 });
		}
	};

	private handleMove = (ll: ILatLng) => {
		if (this.map) {
			console.debug(`[map.events] Move to ${ll.lat},${ll.lng}`);

			this.map.easeTo({ center: [ll.lng, ll.lat] });
		} else {
			console.debug(`[map.events] Move to ${ll.lat},${ll.lng} (unable)`);
		}
	};

	private handleMoveOnce = (ll: ILatLng) => {
		if (this.hasMoved || this.moving) {
			console.debug(`[map.events] Move (once) ignored to ${ll.lat},${ll.lng}`);
			return;
		}

		this.handleMove(ll);
	};

	public mount = (map: Map) => {
		this.map = map;
		map.on('movestart', this.handleMoveStart);
		map.on('moveend', this.handleMoveEnd);
		mapBus.on('fit', this.handleFit);
		mapBus.on('move', this.handleMove);
		mapBus.on('moveOnce', this.handleMoveOnce);
	};

	public unmount = () => {
		this.map?.off('movestart', this.handleMoveStart);
		this.map?.off('moveend', this.handleMoveEnd);
		mapBus.off('fit', this.handleFit);
		mapBus.off('move', this.handleMove);
		mapBus.off('moveOnce', this.handleMoveOnce);
		this.map = undefined;
	};
}
