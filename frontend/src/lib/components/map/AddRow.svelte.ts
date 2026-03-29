import { mapBus } from '$lib/buses/mapBus';
import map, { type LngLat } from 'maplibre-gl';
import { type ILatLng } from '$lib/types';
import { goto, routes } from '$lib/routes';

export type ClickFn = (start: LngLat, end: LngLat) => void;

class AddState {
	active = $state<boolean>(false);
	center = $state<LngLat>();
	start = $state<LngLat>();
	end = $state<LngLat>();

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	line = $state.raw<any>();

	public toggle = (e: Event) => {
		e.preventDefault();
		this.active = !this.active;
	};

	public handleConfirm = async () => {
		if (this.start && this.end) {
			this.active = false;
			this.line = undefined;
			await goto(routes.addRow(this.start, this.end));
		}
	};

	public handleCancel = () => {
		this.active = false;
		this.line = undefined;
	};

	public handleMove = (ll: LngLat) => {
		this.center = ll;
	};

	public handleStart = () => {
		this.start = this.center;
		this.updateLine();
	};

	public handleEnd = () => {
		this.end = this.center;
		this.updateLine();
	};

	public updateLine = () => {
		if (!this.start || !this.end) {
			this.line = undefined;
			return;
		}

		this.line = {
			type: 'Feature',
			geometry: {
				type: 'LineString',
				coordinates: [
					[this.start.lng, this.start.lat],
					[this.end.lng, this.end.lat]
				]
			}
		};
	};

	private handleCenter = (ll: ILatLng) => {
		this.center = new map.LngLat(ll.lng, ll.lat);
	};

	public onMount = () => {
		mapBus.on('center', this.handleCenter);

		return () => {
			mapBus.off('center', this.handleCenter);
		};
	};
}

export const addState = new AddState();
