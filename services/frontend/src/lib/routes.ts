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
	learn: () => '/tools/learn',
	map: () => '/',
	alertPreview: (id: string) => `/alert/${id}/preview`,
	modeMapper: () => '/mode/mapper',
	profile: () => '/profile',
	streetReport: (street?: string) => build('/tools/street-report', { address: street }),
	settings: () => '/profile/settings',
	search: () => '/search',
	searchResults: (query: string) => build('/search/results', { query }),
	searchAddress: (query: string) => build('/search/results', { query: `addr:"${query}"` }),
	searchQuery: (query: string) => (query ? build('/search/results', { query }) : '/search/results'),
	searchSpecies: (query: string) => build('/search/results', { query: `species:"${query}"` }),
	searchState: (query: string) => build('/search/results', { query: `state:"${query}"` }),
	stats: () => '/tools/stats',
	data: () => '/export',
	panorama: (id: string) => `/panoramas/${id}`,
	panoramaDetect: (id: string) => `/panoramas/${id}/detect`,
	statsCircumference: () => '/tools/stats/circumference',
	statsDiameter: () => '/tools/stats/diameter',
	statsHeight: () => '/tools/stats/height',
	statsSpecies: () => '/tools/stats/species',
	statsState: () => '/tools/stats/state',
	statsStreets: () => '/tools/stats/streets',
	addTree: () => '/add/tree',
	uploads: () => '/profile/uploads',
	addRow: () => '/add/row',
	treeDead: (id: string) => `/tree/${id}/dead`,
	treeDelete: (id: string) => `/tree/${id}/delete`,
	treeDetails: (id: string) => `/tree/${id}`,
	treeStreetView: (id: string) => `/tree/${id}/360`,
	treeDiameter: (id: string) => `/tree/${id}/diameter`,
	treeCircumference: (id: string) => `/tree/${id}/circumference`,
	treeEdit: (id: string) => `/tree/${id}/edit`,
	treeHeight: (id: string) => `/tree/${id}/height`,
	treeHistory: (id: string) => `/tree/${id}/history`,
	treeMove: (id: string) => `/tree/${id}/move`,
	treeComments: (id: string) => `/tree/${id}/comments`,
	treeObservations: (id: string) => `/tree/${id}/observations`,
	treePhotos: (id: string) => `/tree/${id}/photos`,
	treeReplace: (id: string) => `/tree/${id}/replace`,
	addWater: () => '/add/water',
	waterDetails: (id: string) => `/water/${id}`,
	waterMove: (id: string) => `/water/${id}/move`,
	privacy: () => '/privacy',
	tools: () => '/tools',
	toolsRange: () => '/tools/range',
	toolsRangeEnter: () => '/tools/range/enter',
	toolsRangeSubmit: () => '/tools/range/submit',
	toolsMerge: () => '/tools/merge',
	toolsMergePair: (src: string, dst: string) => `/tools/merge/${src}/${dst}`,
	toolsUnknownSpecies: () => '/tools/unknown-species'
};
