export { goto } from '$app/navigation';
import type { ILatLng } from '$lib/types';

type Params = {
	[key: string]: string | undefined | null;
};

const build = (path: string, params: Params) => {
	const qs = new URLSearchParams();

	for (const [k, v] of Object.entries(params)) {
		if (v !== undefined && v !== null) {
			qs.append(k, v);
		}
	}

	return `${path}?${qs.toString()}`;
};

export const routes = {
	changedTrees: () => '/updates/changes',
	comments: () => '/updates/comments',
	file: (id: string) => `https://yerevan.treemaps.app/v1/files/${id}.jpg`,
	home: () => '/',
	learn: () => '/learn',
	map: () => '/map',
	mapPreview: (id: string, search?: string | undefined | null) =>
		build('/map', {
			preview: id,
			q: search
		}),
	modeMapper: () => '/mode/mapper',
	newTrees: () => '/updates/new',
	profile: () => '/profile',
	streetReport: (street: string) => build('/report', { address: street }),
	settings: () => '/settings',
	search: () => '/search',
	searchAddress: (query: string) => `/map?q=addr:"${query}"`,
	searchQuery: (query: string) => `/map?q=${query}`,
	searchSpecies: (query: string) => `/map?q=species:"${query}"`,
	searchState: (query: string) => `/map?q=state:"${query}"`,
	stats: () => '/stats',
	statsCircumference: () => '/stats/circumference',
	statsDiameter: () => '/stats/diameter',
	statsHeight: () => '/stats/height',
	statsMismatch: () => '/stats/species-mismatch',
	statsSpecies: () => '/stats/species',
	statsState: () => '/stats/state',
	statsStreets: () => '/stats/streets',
	treeAdd: (lat: number, lng: number) => `/add?lat=${lat}&lng=${lng}`,
	addRow: (start: ILatLng, end: ILatLng) =>
		build('/add/row', {
			alat: start.lat.toString(),
			alng: start.lng.toString(),
			blat: end.lat.toString(),
			blng: end.lng.toString()
		}),
	treeComments: (id: string) => `/tree/${id}/comments`,
	treeDead: (id: string) => `/tree/${id}/dead`,
	treeDelete: (id: string) => `/tree/${id}/delete`,
	treeDetails: (id: string) => `/tree/${id}`,
	treeStreetView: (id: string) => `/tree/${id}/360`,
	treeDiameter: (id: string) => `/tree/${id}/diameter`,
	treeCircumference: (id: string) => `/tree/${id}/circumference`,
	treeEdit: (id: string) => `/tree/${id}/edit`,
	treeHeight: (id: string) => `/tree/${id}/height`,
	treeHistory: (id: string) => `/tree/${id}/history`,
	treeMap: (id: string) => `/tree/${id}/map`,
	treeMeasure: (id: string) => `/tree/${id}/measure`,
	treeMove: (id: string) => `/tree/${id}/move`,
	treeReplace: (id: string) => `/tree/${id}/replace`,
	treeUploadPhotos: (id: string) => `/tree/${id}/upload`
};
