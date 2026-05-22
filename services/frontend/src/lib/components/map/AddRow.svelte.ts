import { goto, routes } from '$lib/routes';
import { mapState } from './MapLibre.svelte.ts';
import type { ILatLng, ILatLon, IAddTreesRequest } from '$lib/types';
import { getDistance } from '$lib/utils';
import { spreadDots } from '$lib/map';
import { addTree } from '$lib/api/trees';
import { showError } from '$lib/errors';
import { mapBus } from '$lib/buses/mapBus';

class AddState {
	active = $state<boolean>(false);
	saving = $state<boolean>(false);
	count = $state<number>(2);
	pointA = $state<ILatLng | null>(null);
	pointB = $state<ILatLng | null>(null);

	distance = $derived(this.pointA && this.pointB ? getDistance(this.pointA, this.pointB) : 0);
	step = $derived(
		this.distance > 0 && this.count > 1
			? Math.round((this.distance / (this.count - 1)) * 10) / 10
			: 0
	);

	previewData = $derived.by(() => {
		if (!this.pointA || !this.pointB) return undefined;

		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const res: any = {
			type: 'FeatureCollection',
			features: []
		};

		res.features.push({
			type: 'Feature',
			geometry: {
				type: 'LineString',
				coordinates: [
					[this.pointA.lng, this.pointA.lat],
					[this.pointB.lng, this.pointB.lat]
				]
			}
		});

		for (const dot of spreadDots(this.pointA, this.pointB, this.count)) {
			res.features.push({
				type: 'Feature',
				geometry: {
					type: 'Point',
					coordinates: [dot.lng, dot.lat]
				}
			});
		}

		return res;
	});

	public toggle = (e: Event) => {
		e.preventDefault();
		goto(routes.addRow());
	};

	public setPointA = () => {
		this.pointA = {
			lat: mapState.center.lat,
			lng: mapState.center.lng
		};
	};

	public setPointB = () => {
		this.pointB = {
			lat: mapState.center.lat,
			lng: mapState.center.lng
		};
	};

	public handleCountChange = (value: number) => {
		this.count = value;
	};

	public handleConfirm = async () => {
		if (!this.pointA || !this.pointB) {
			return;
		}

		const points: ILatLon[] = spreadDots(this.pointA, this.pointB, this.count).map((point) => ({
			lat: point.lat,
			lon: point.lng
		}));

		const req = {
			points,
			species: '',
			height: null,
			diameter: null,
			circumference: null,
			state: 'unknown',
			notes: null,
			year: null,
			files: []
		} as IAddTreesRequest;

		try {
			this.saving = true;

			const { status, data: d, error: e } = await addTree(req);

			if (status >= 200 && status < 300 && d) {
				const id = d.trees[0].id;
				goto(routes.mapPreview(id));
				this.reset();
			} else if (e) {
				console.error(`Error ${status} adding trees.`, e);
				showError(e.description);
			}
		} finally {
			this.saving = false;
			mapBus.emit('reload');
		}
	};

	public handleCancel = () => {
		this.reset();
		goto(routes.map());
	};

	public reset = () => {
		this.pointA = null;
		this.pointB = null;
		this.count = 2;
		this.saving = false;
	};

	public onMount = () => {
		return () => {};
	};
}

export const addState = new AddState();
