export const routes = {
	changedTrees: () => '/updates/changes',
	comments: () => '/updates/comments',
	file: (id: string) => `https://yerevan.treemaps.app/v1/files/${id}.jpg`,
	home: () => '/',
	newTrees: () => '/updates/new',
	learn: () => '/learn',
	search: '/search',
	searchQuery: (query: string) => `/map?q=${query}`,
	searchSpecies: (query: string) => `/map?q=species:"${query}"`,
	searchAddress: (query: string) => `/map?q=addr:"${query}"`,
	stats: () => '/stats',
	statsSpecies: () => '/stats/species',
	statsHeight: () => '/stats/height',
	statsDiameter: () => '/stats/diameter',
	statsCircumference: () => '/stats/circumference',
	statsStreets: () => '/stats/streets',
	statsMismatch: () => '/stats/species-mismatch',
	treeAdd: (lat: number, lng: number) => `/add?lat=${lat}&lng=${lng}`,
	treeComments: (id: string) => `/tree/${id}/comments`,
	treeDetails: (id: string) => `/tree/${id}`,
	treeEdit: (id: string) => `/tree/${id}/edit`,
	treeHistory: (id: string) => `/tree/${id}/history`,
	treeMap: (id: string) => `/tree/${id}/map`,
	treeUploadPhotos: (id: string) => `/tree/${id}/upload`
};
