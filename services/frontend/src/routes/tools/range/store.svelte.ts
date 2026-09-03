import type { ILatLng } from '$lib/types';
import { ls } from '$lib/utils/localStorage';

export interface ITriangulatedTree {
	id: string;
	lat: number;
	lng: number;
	createdAt: number;
}

export class RangeToolStore {
	gcps = $state<Array<ILatLng | null>>([null, null, null, null]);
	trees = $state<ITriangulatedTree[]>([]);
	selectedTree = $state<string | undefined>(undefined);

	constructor() {
		this.init();
	}

	init = () => {
		this.load();
	};

	load = () => {
		const savedGcps = ls.read('range_tool_gcps');
		if (Array.isArray(savedGcps)) {
			this.gcps = [0, 1, 2, 3].map((i) => {
				const g = savedGcps[i];
				if (!g || Number.isNaN(g.lat) || Number.isNaN(g.lng) || (g.lat === 0 && g.lng === 0)) {
					return null;
				}
				return g;
			});
		}

		const savedTrees = ls.read('range_tool_trees');
		if (Array.isArray(savedTrees)) {
			this.trees = savedTrees.filter(
				(t) => t && typeof t.id === 'string' && !Number.isNaN(t.lat) && !Number.isNaN(t.lng)
			);
		}
	};

	setGcp = (index: number, val: ILatLng) => {
		const next = [...this.gcps];
		next[index] = val;
		this.gcps = next;
		ls.write('range_tool_gcps', this.gcps);
	};

	clearGcp = (index: number) => {
		const next = [...this.gcps];
		for (let i = index; i < next.length; i++) {
			next[i] = null;
		}
		this.gcps = next;
		ls.write('range_tool_gcps', this.gcps);
	};

	setSelectedTree = (id: string | undefined) => {
		this.selectedTree = id;
	};

	addTree = (location: ILatLng) => {
		if (!location || Number.isNaN(location.lat) || Number.isNaN(location.lng)) return;
		const newTree: ITriangulatedTree = {
			id: crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(36).substring(2),
			lat: location.lat,
			lng: location.lng,
			createdAt: Date.now()
		};
		this.trees = [...this.trees, newTree];
		ls.write('range_tool_trees', this.trees);
	};

	removeTree = (id: string) => {
		this.trees = this.trees.filter((t) => t.id !== id);
		if (this.selectedTree === id) {
			this.selectedTree = undefined;
		}
		ls.write('range_tool_trees', this.trees);
	};

	clearTrees = () => {
		this.trees = [];
		this.selectedTree = undefined;
		ls.write('range_tool_trees', this.trees);
	};

	validGcps = $derived(
		this.gcps.filter(
			(g): g is ILatLng =>
				g !== null && !Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)
		)
	);

	validCount = $derived(this.validGcps.length);

	canContinue = $derived(this.validCount >= 2);
}

export const rangeStore = new RangeToolStore();
