export const routes = {
	file: (id: string) => `https://yerevan.treemaps.app/v1/files/${id}.jpg`,
	treeComments: (id: string) => `/tree/${id}/comments`,
	treeDetails: (id: string) => `/tree/${id}/`,
	treeHistory: (id: string) => `/tree/${id}/history`,
	treeMap: (id: string) => `/tree/${id}/map`
};
