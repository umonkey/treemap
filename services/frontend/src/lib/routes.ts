export { goto } from '$app/navigation';
import { config } from '$lib/env';

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

	const query = qs.toString();

	if (!query) {
		return path;
	}

	return `${path}?${qs.toString()}`;
};

export const routes = {
	treeUpdates: () => '/updates',
	treeSaved: () => '/saved',
	comments: () => '/updates/comments',
	file: (id: string) => `${config.fileBaseUrl}${id}.jpg`,
	home: () => '/',
	learn: () => '/learn',
	map: () => '/',
	mapPreview: (id: string) => `/tree/${id}/preview`,
	alertPreview: (id: string) => `/alert/${id}/preview`,
	modeMapper: () => '/mode/mapper',
	profile: () => '/profile',
	streetReport: (street?: string) => build('/report', { address: street }),
	settings: () => '/profile/settings',
	search: () => '/search',
	searchAddress: (query: string) => `/?q=addr:"${query}"`,
	searchQuery: (query: string) => (query ? `/?q=${query}` : '/'),
	searchSpecies: (query: string) => `/?q=species:"${query}"`,
	searchState: (query: string) => `/?q=state:"${query}"`,
	stats: () => '/stats',
	panorama: (id: string) => `/panoramas/${id}`,
	statsCircumference: () => '/stats/circumference',
	statsDiameter: () => '/stats/diameter',
	statsHeight: () => '/stats/height',
	statsMismatch: () => '/stats/species-mismatch',
	statsSpecies: () => '/stats/species',
	statsState: () => '/stats/state',
	statsStreets: () => '/stats/streets',
	treeAdd: () => `/add`,
	uploads: () => '/profile/uploads',
	addRow: () => '/add-row',
	treeDead: (id: string) => `/tree/${id}/dead`,
	treeDelete: (id: string) => `/tree/${id}/delete`,
	treeDetails: (id: string, imageId?: string) => build(`/tree/${id}`, { image: imageId }),
	treeStreetView: (id: string) => `/tree/${id}/360`,
	treeDiameter: (id: string) => `/tree/${id}/diameter`,
	treeCircumference: (id: string) => `/tree/${id}/circumference`,
	treeEdit: (id: string) => `/tree/${id}/edit`,
	treeHeight: (id: string) => `/tree/${id}/height`,
	treeHistory: (id: string) => `/tree/${id}/history`,
	treeMove: (id: string) => `/tree/${id}/move`,
	treeObservations: (id: string) => `/tree/${id}/observations`,
	treeReplace: (id: string) => `/tree/${id}/replace`,
	treeUploadPhotos: (id: string) => `/tree/${id}/upload`,
	layers: () => '/layers',
	privacy: () => '/privacy'
};
