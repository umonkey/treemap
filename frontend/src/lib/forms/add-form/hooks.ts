import type { IAddTreesRequest, ITree, ILatLng } from '$lib/types';
import { apiClient } from '$lib/api';
import { get } from 'svelte/store';
import { isMapperMode } from '$lib/stores/modeStore';
import { routes, goto } from '$lib/routes';
import { toast } from '@zerodevx/svelte-toast';
import { writable } from 'svelte/store';

const DEFAULT_TREE = {
	id: '',
	lat: 40.181389,
	lon: 44.514444,
	address: 'Armenia',
	osm_id: null,
	species: '',
	notes: null,
	height: null,
	circumference: null,
	diameter: null,
	state: 'unknown',
	added_at: 0,
	updated_at: 0,
	added_by: '',
	year: null,
	replaces: null,
	replaced_by: null,
	files: [],
	users: []
} as ITree;

export const hooks = () => {
	const tree = writable<ITree>(DEFAULT_TREE);
	const location = writable<ILatLng>({ lat: 0, lng: 0 });
	const loading = writable<boolean>(true);
	const saving = writable<boolean>(false);

	// Set when the file picker component is uploading files, cannot submit.
	const uploading = writable<boolean>(false);

	// Set when a file was uploaded, contains file ids.
	const uploads = writable<string[]>([]);

	const reload = (lat: number, lng: number) => {
		location.set({ lat, lng });

		tree.set({
			...DEFAULT_TREE,
			lat,
			lon: lng
		});

		loading.set(false);
	};

	const handleSpeciesChange = (value: string) => {
		tree.update((t) => {
			if (t) {
				t.species = value;
			}

			return t;
		});
	};

	const handleHeightChange = (value: number) => {
		tree.update((t) => {
			if (t) {
				t.height = value;
			}

			return t;
		});
	};

	const handleDiameterChange = (value: number) => {
		tree.update((t) => {
			if (t) {
				t.diameter = value;
			}

			return t;
		});
	};

	const handleCircumferenceChange = (value: number) => {
		tree.update((t) => {
			if (t) {
				t.circumference = value;
			}

			return t;
		});
	};

	const handleStateChange = (value: string) => {
		tree.update((t) => {
			if (t) {
				t.state = value;
			}

			return t;
		});
	};

	const handleNotesChange = (value: string) => {
		tree.update((t) => {
			if (t) {
				t.notes = value;
			}

			return t;
		});
	};

	const handleYearChange = (value: number) => {
		tree.update((t) => {
			if (t) {
				t.year = value;
			}

			return t;
		});
	};

	const handleLocationChange = (value: ILatLng) => {
		tree.update((t) => {
			if (t) {
				t.lat = value.lat;
				t.lon = value.lng;
			}

			return t;
		});
	};

	const handleConfirm = () => {
		const t = get(tree);

		const req = {
			points: [
				{
					lat: t.lat,
					lon: t.lon
				}
			],
			species: t.species,
			height: t.height,
			diameter: t.diameter,
			circumference: t.circumference,
			state: t.state,
			notes: t.notes,
			year: t.year,
			files: get(uploads)
		} as IAddTreesRequest;

		apiClient.addTree(req).then((res) => {
			if (res.status >= 200 && res.status < 400 && res.data) {
				toast.push('Tree tree.');

				if (get(isMapperMode)) {
					goto(routes.mapPreview(res.data.trees[0].id));
				} else {
					goto(routes.treeDetails(res.data.trees[0].id));
				}
			} else {
				console.error(`Error ${res.status} updating tree.`);
				toast.push('Error updating tree.');
			}
		});
	};

	const handleCancel = () => {
		goto(routes.map());
	};

	const handleUploading = (value: boolean) => {
		uploading.set(value);
	};

	// Receives ids of all currently uploaded files.
	const handleUploaded = (value: string[]) => {
		uploads.set(value);
	};

	return {
		loading,
		location,
		tree,
		reload,
		handleSpeciesChange,
		handleHeightChange,
		handleDiameterChange,
		handleCircumferenceChange,
		handleStateChange,
		handleNotesChange,
		handleLocationChange,
		handleYearChange,
		handleConfirm,
		handleCancel,
		handleUploading,
		handleUploaded,
		saving,
		uploading
	};
};
