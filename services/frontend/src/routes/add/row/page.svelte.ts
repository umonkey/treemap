import { addTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { spreadDots } from '$lib/map';
import { goto, routes } from '$lib/routes';
import type { IAddTreesRequest, ILatLng, ILatLon } from '$lib/types';
import { getDistance } from '$lib/utils';

class PageState {
	count = $state<number>(3);
	saving = $state<boolean>(false);

	species = $state<string | null>(null);
	height = $state<number | null>(null);
	diameter = $state<number | null>(null);
	circumference = $state<number | null>(null);
	state = $state<string | null>(null);
	year = $state<number | null>(null);
	notes = $state<string | null>(null);

	start = $state<ILatLng | null>(null);
	end = $state<ILatLng | null>(null);

	distance = $derived(this.start && this.end ? getDistance(this.start, this.end) : 0);
	step = $derived(
		this.distance > 0 && this.count > 0 ? Math.round((this.distance / this.count) * 10) / 10 : 0
	);

	init(start: ILatLng, end: ILatLng) {
		this.start = start;
		this.end = end;
	}

	handleCountChange = (value: number) => {
		console.debug(`[map] Tree count changed to ${value}`);
		this.count = value;
	};

	handleSpeciesChange = (value: string | null) => {
		console.debug(`[map] Species changed to ${value}`);
		this.species = value;
	};

	handleHeightChange = (value: number | null) => {
		console.debug(`[map] Height changed to ${value}`);
		this.height = value;
	};

	handleDiameterChange = (value: number | null) => {
		console.debug(`[map] Diameter changed to ${value}`);
		this.diameter = value;
	};

	handleCircumferenceChange = (value: number | null) => {
		console.debug(`[map] Circumference changed to ${value}`);
		this.circumference = value;
	};

	handleStateChange = (value: string | null) => {
		console.debug(`[map] State changed to ${value}`);
		this.state = value;
	};

	handleYearChange = (value: number | null) => {
		console.debug(`[map] Year changed to ${value}`);
		this.year = value;
	};

	handleNotesChange = (value: string | null) => {
		console.debug(`[map] Notes changed to ${value}`);
		this.notes = value;
	};

	handleConfirm = async () => {
		if (!this.start || !this.end) {
			return;
		}

		const points: ILatLon[] = spreadDots(this.start, this.end, this.count).map((point) => ({
			lat: point.lat,
			lon: point.lng
		}));

		const req = {
			points,
			species: this.species ?? 'Unknown',
			height: this.height,
			diameter: this.diameter,
			circumference: this.circumference,
			state: this.state ?? 'unknown',
			notes: this.notes,
			year: this.year,
			files: []
		} as IAddTreesRequest;

		try {
			this.saving = true;

			const { status, data: d, error: e } = await addTree(req);

			if (status >= 200 && status < 300 && d) {
				const id = d.trees[0].id;
				goto(routes.mapPreview(id));
			} else if (e) {
				console.error(`Error ${status} adding trees.`, e);
				showError(e.description);
			}
		} finally {
			this.saving = false;
			mapBus.emit('reload');
		}
	};

	handleCancel = () => {
		goto(routes.map());
	};
}

export const pageState = new PageState();
