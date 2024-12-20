export const routes = {
	file: (id: string) => `https://yerevan.treemaps.app/v1/files/${id}.jpg`,
	search: '/search',
	searchQuery: (query: string) => `/map?q=${query}`,
	treeAdd: (lat: number, lng: number) => `/add?lat=${lat}&lng=${lng}`,
	treeComments: (id: string) => `/tree/${id}/comments`,
	treeDetails: (id: string) => `/tree/${id}`,
	treeEdit: (id: string) => `/tree/${id}/edit`,
	treeHistory: (id: string) => `/tree/${id}/history`,
	treeMap: (id: string) => `/tree/${id}/map`,
	treeUploadPhotos: (id: string) => `/tree/${id}/upload`,
	comments: () => '/updates/comments',
	newTrees: () => '/updates/new'
};
