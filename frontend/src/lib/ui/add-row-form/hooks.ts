import { toast } from '@zerodevx/svelte-toast';
import { apiClient } from '$lib/api';
import { get, writable } from 'svelte/store';
import { getDistance } from '$lib/utils';
import type { ILatLng, ILatLon, IAddTreesRequest } from '$lib/types';
import { goto, routes } from '$lib/routes';
import { spreadDots } from '$lib/map';

export const hooks = ({ start, end }: { start: ILatLng; end: ILatLng }) => {
	const count = writable<number>(3);
	const saving = writable<boolean>(false);
	const step = writable<number>(0);

	const species = writable<string | null>(null);
	const height = writable<number | null>(null);
	const diameter = writable<number | null>(null);
	const circumference = writable<number | null>(null);
	const state = writable<string | null>(null);
	const year = writable<number | null>(null);
	const notes = writable<string | null>(null);

	const distance = getDistance(start, end);

	const updateStep = () => {
		step.set(Math.round((distance / get(count)) * 10) / 10);
		console.debug(`[map] Row length=${distance}, count=${get(count)}, step=${get(step)}`);
	};

	const handleCountChange = (value: number) => {
		console.debug(`[map] Tree count changed to ${value}`);
		count.set(value);
		updateStep();
	};

	const handleSpeciesChange = (value: string | null) => {
		console.debug(`[map] Species changed to ${value}`);
		species.set(value);
	};

	const handleHeightChange = (value: number | null) => {
		console.debug(`[map] Height changed to ${value}`);
		height.set(value);
	};

	const handleDiameterChange = (value: number | null) => {
		console.debug(`[map] Diameter changed to ${value}`);
		diameter.set(value);
	};

	const handleCircumferenceChange = (value: number | null) => {
		console.debug(`[map] Circumference changed to ${value}`);
		circumference.set(value);
	};

	const handleStateChange = (value: string | null) => {
		console.debug(`[map] State changed to ${value}`);
		state.set(value);
	};

	const handleYearChange = (value: number | null) => {
		console.debug(`[map] Year changed to ${value}`);
		year.set(value);
	};

	const handleNotesChange = (value: string | null) => {
		console.debug(`[map] Notes changed to ${value}`);
		notes.set(value);
	};

	const handleConfirm = async () => {
		const points: ILatLon[] = spreadDots(start, end, get(count)).map((point) => ({
			lat: point.lat,
			lon: point.lng
		}));

		const req = {
			points,
			species: get(species) ?? 'Unknown',
			height: get(height),
			diameter: get(diameter),
			circumference: get(circumference),
			state: get(state) ?? 'unknown',
			notes: get(notes),
			year: get(year),
			files: []
		} as IAddTreesRequest;

		try {
			saving.set(true);

			const { status, data: d, error: e } = await apiClient.addTree(req);

			if (status >= 200 && status < 300 && d) {
				toast.push('Trees added.');
				goto(routes.mapPreview(d.trees[0].id));
			} else if (e) {
				console.error(`Error ${status} adding trees.`);
				toast.push('Error adding trees.');
			}
		} finally {
			saving.set(false);
		}
	};

	const handleCancel = () => {
		goto(routes.map());
	};

	updateStep();

	return {
		count,
		step,
		distance,
		saving,
		handleCountChange,
		handleConfirm,
		handleCancel,
		handleSpeciesChange,
		handleHeightChange,
		handleDiameterChange,
		handleCircumferenceChange,
		handleStateChange,
		handleYearChange,
		handleNotesChange
	};
};
