import type { IMeResponse, ITree } from '$lib/types';

export const DEFAULT_MAP_CENTER = [40.181389, 44.514444];

export const MAX_BOUNDS = [
	[
		38.82, // south
		43.42 // west
	],
	[
		41.32, // north
		46.65 // east
	]
];

export const DEFAULT_TREE = {
	id: 'tree1',
	lat: 40.181389,
	lon: 44.514444,
	address: 'Yerevan, Armenia',
	osm_id: null,
	species: 'Populus nigra',
	notes: null,
	height: null,
	circumference: null,
	diameter: null,
	state: 'healthy',
	added_at: 0,
	updated_at: 0,
	added_by: 'user1',
	year: null,
	files: []
} as ITree;

export const DEFAULT_ME = {
	name: 'user1',
	picture: 'https://example.com/foobar.jpg',
	trees_count: 0,
	comments_count: 0,
	updates_count: 0,
	files_count: 0
} as IMeResponse;
