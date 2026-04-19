import { addTree } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import type { IAddTreesRequest, ILatLng, ITree } from '$lib/types';

const DEFAULT_TREE: ITree = {
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
	thumbnail_id: null,
	files: []
};

class PageState {
	tree = $state<ITree>({ ...DEFAULT_TREE });
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);

	location = $derived<ILatLng>({
		lat: this.tree.lat,
		lng: this.tree.lon
	});

	reload = (lat: number, lng: number) => {
		this.tree = {
			...DEFAULT_TREE,
			lat,
			lon: lng
		};

		this.loading = false;
	};

	handleSpeciesChange = (value: string) => {
		this.tree.species = value;
	};

	handleHeightChange = (value: number) => {
		this.tree.height = value;
	};

	handleDiameterChange = (value: number) => {
		this.tree.diameter = value;
	};

	handleCircumferenceChange = (value: number) => {
		this.tree.circumference = value;
	};

	handleStateChange = (value: string) => {
		this.tree.state = value;
	};

	handleNotesChange = (value: string) => {
		this.tree.notes = value;
	};

	handleYearChange = (value: number) => {
		this.tree.year = value;
	};

	handleLocationChange = (value: ILatLng) => {
		this.tree.lat = value.lat;
		this.tree.lon = value.lng;
	};

	handleConfirm = () => {
		const req = {
			points: [
				{
					lat: this.tree.lat,
					lon: this.tree.lon
				}
			],
			species: this.tree.species,
			height: this.tree.height,
			diameter: this.tree.diameter,
			circumference: this.tree.circumference,
			state: this.tree.state,
			notes: this.tree.notes,
			year: this.tree.year,
			files: []
		} as IAddTreesRequest;

		this.saving = true;

		addTree(req)
			.then((res) => {
				if (res.status >= 200 && res.status < 400 && res.data) {
					const id = res.data.trees[0].id;

					if (this.tree.state === 'healthy' || this.tree.state === 'dead') {
						goto(routes.treeObservations(id));
					} else if (this.tree.state === 'stump' || this.tree.state === 'gone') {
						goto(routes.treeUploadPhotos(id));
					} else {
						goto(routes.mapPreview(id));
					}
				} else {
					console.error(`Error ${res.status} adding tree.`, res);
					showError(res.error?.description || `Error ${res.status} adding tree.`);
				}
			})
			.finally(() => {
				this.saving = false;
				mapBus.emit('reload');
			});
	};

	handleCancel = () => {
		goto(routes.map());
	};
}

export const pageState = new PageState();
