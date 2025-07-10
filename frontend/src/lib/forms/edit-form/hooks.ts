import { writable } from 'svelte/store';
import type { ITree, ILatLng } from '$lib/types';
import { apiClient } from '$lib/api';
import { routes, goto } from '$lib/routes';
import { DEFAULT_TREE } from '$lib/constants';
import { get } from 'svelte/store';
import { toast } from '@zerodevx/svelte-toast';

export const hooks = () => {
	const tree = writable<ITree>(DEFAULT_TREE);
	const updated = writable<ITree>(DEFAULT_TREE);
	const loading = writable<boolean>(true);
	const saving = writable<boolean>(false);

	const reload = (id: string) => {
		apiClient.getTree(id).then((res) => {
			if (res.status === 200 && res.data) {
				console.debug('[edit form] Tree data loaded.');
				tree.set(res.data);
				updated.set(res.data);
				loading.set(false);
			}
		});
	};

	const handleSpeciesChange = (value: string) => {
		updated.update((t) => {
			if (t) {
				t.species = value;
			}

			return t;
		});
	};

	const handleHeightChange = (value: number) => {
		updated.update((t) => {
			if (t) {
				t.height = value;
			}

			return t;
		});
	};

	const handleDiameterChange = (value: number) => {
		updated.update((t) => {
			if (t) {
				t.diameter = value;
			}

			return t;
		});
	};

	const handleCircumferenceChange = (value: number) => {
		updated.update((t) => {
			if (t) {
				t.circumference = value;
			}

			return t;
		});
	};

	const handleStateChange = (value: string) => {
		updated.update((t) => {
			if (t) {
				t.state = value;
			}

			return t;
		});
	};

	const handleNotesChange = (value: string) => {
		updated.update((t) => {
			if (t) {
				t.notes = value;
			}

			return t;
		});
	};

	const handleYearChange = (value: number) => {
		updated.update((t) => {
			if (t) {
				t.year = value;
			}

			return t;
		});
	};

	const handleLocationChange = (value: ILatLng) => {
		updated.update((t) => {
			if (t) {
				t.lat = value.lat;
				t.lon = value.lng;
			}

			return t;
		});
	};

	const handleAddressChange = (value: string) => {
		updated.update((t) => {
			if (t) {
				t.address = value || null;
				console.debug(`[edit form] Address updated to: ${value}`);
			}

			return t;
		});
	};

	const handleConfirm = () => {
		const u = get(updated);

		apiClient
			.updateTree(u.id, {
				...get(tree),
				species: u.species,
				height: u.height,
				diameter: u.diameter,
				circumference: u.circumference,
				state: u.state,
				lat: u.lat,
				lon: u.lon,
				notes: u.notes,
				year: u.year
			})
			.then((res) => {
				if (res.status >= 200 && res.status < 400) {
					toast.push('Tree updated.');
					goto(routes.mapPreview(u.id));
				} else {
					console.error(`Error ${res.status} updating tree.`);
					toast.push('Error updating tree.');
				}
			});
	};

	const handleCancel = () => {
		goto(routes.treeDetails(get(tree).id));
	};

	return {
		tree,
		updated,
		reload,
		handleSpeciesChange,
		handleHeightChange,
		handleDiameterChange,
		handleCircumferenceChange,
		handleStateChange,
		handleNotesChange,
		handleLocationChange,
		handleYearChange,
		handleAddressChange,
		handleConfirm,
		handleCancel,
		loading,
		saving
	};
};
