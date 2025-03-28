export { goto } from '$app/navigation';

export const routes = {
	changedTrees: () => '/updates/changes',
	comments: () => '/updates/comments',
	file: (id: string) => `https://yerevan.treemaps.app/v1/files/${id}.jpg`,
	home: () => '/',
	learn: () => '/learn',
	map: () => '/map',
	mapPreview: (id: string) => `/map?preview=${id}`,
	newTrees: () => '/updates/new',
	search: '/search',
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
	treeComments: (id: string) => `/tree/${id}/comments`,
	treeDetails: (id: string) => `/tree/${id}`,
	treeEdit: (id: string) => `/tree/${id}/edit`,
	treeHistory: (id: string) => `/tree/${id}/history`,
	treeMap: (id: string) => `/tree/${id}/map`,
	treeMeasure: (id: string) => `/tree/${id}/measure`,
	treeUploadPhotos: (id: string) => `/tree/${id}/upload`
};
