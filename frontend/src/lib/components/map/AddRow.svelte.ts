import { type LngLat } from 'maplibre-gl';

export type ClickFn = (start: LngLat, end: LngLat) => void;

class AddState {
	active = $state<boolean>(false);
	center = $state<LngLat>();
	start = $state<LngLat>();
	end = $state<LngLat>();

	line = $state();

	onConfirm = $state<ClickFn>(() => {});

	public toggle = (e: Event) => {
		e.preventDefault();
		this.active = !this.active;
	};

	public handleConfirm = () => {
		if (this.start && this.end) {
			this.onConfirm(this.start, this.end);
			this.active = false;
			this.line = undefined;
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
}

export const addState = new AddState();
