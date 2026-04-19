import { getTree, updateTree } from '$lib/api/trees';
import { DEFAULT_TREE } from '$lib/constants';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import type { ILatLng, ITree } from '$lib/types';

class PageState {
	tree = $state<ITree>(DEFAULT_TREE);
	updated = $state<ITree>(DEFAULT_TREE);
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);
	currentId = $state<string>('');

	reload = async (id: string) => {
		if (this.currentId === id) return;
		this.currentId = id;
		this.loading = true;

		try {
			const res = await getTree(id);
			if (res.status === 200 && res.data) {
				console.debug('[edit form] Tree data loaded.');
				this.tree = res.data;
				this.updated = { ...res.data };
				this.loading = false;
			}
		} catch (err) {
			console.error('Failed to reload tree data:', err);
		}
	};

	handleSpeciesChange = (value: string) => {
		this.updated.species = value;
	};

	handleHeightChange = (value: number) => {
		this.updated.height = value;
	};

	handleDiameterChange = (value: number) => {
		this.updated.diameter = value;
	};

	handleCircumferenceChange = (value: number) => {
		this.updated.circumference = value;
	};

	handleStateChange = (value: string) => {
		this.updated.state = value;
	};

	handleNotesChange = (value: string) => {
		this.updated.notes = value;
	};

	handleYearChange = (value: number) => {
		this.updated.year = value;
	};

	handleLocationChange = (value: ILatLng) => {
		this.updated.lat = value.lat;
		this.updated.lon = value.lng;
	};

	handleAddressChange = (value: string) => {
		this.updated.address = value || null;
		console.debug(`[edit form] Address updated to: ${value}`);
	};

	handleConfirm = async () => {
		this.saving = true;

		try {
			const res = await updateTree(this.updated.id, {
				...this.tree,
				species: this.updated.species,
				height: this.updated.height,
				diameter: this.updated.diameter,
				circumference: this.updated.circumference,
				state: this.updated.state,
				lat: this.updated.lat,
				lon: this.updated.lon,
				notes: this.updated.notes,
				year: this.updated.year
			});

			if (res.status >= 200 && res.status < 400) {
				goto(routes.mapPreview(this.updated.id));
			} else {
				console.error(`Error ${res.status} updating tree.`, res);
				showError(res.error?.description || `Error ${res.status} updating tree.`);
			}
		} catch (err) {
			console.error('Failed to update tree:', err);
			showError('Failed to update tree.');
		} finally {
			this.saving = false;
		}
	};

	handleCancel = async () => {
		await goto(routes.treeDetails(this.tree.id));
	};
}

export const pageState = new PageState();
