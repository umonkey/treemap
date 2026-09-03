import { routes, goto } from '$lib/routes';
import type { ILatLng } from '$lib/types';
import { ls } from '$lib/utils/localStorage';
import { triangulateTree } from '$lib/utils/triangulation';
import { locationStore } from '$lib/stores/locationStore';
import { get } from 'svelte/store';

export class RangeEnterState {
	gcps = $state<Array<ILatLng & { radius: number; index: number }>>([]);
	radii = $state<number[]>([]);

	constructor() {
		const saved = ls.read('range_tool_gcps');
		if (!Array.isArray(saved)) {
			goto(routes.toolsRange());
			return;
		}

		const validWithIndices: Array<ILatLng & { radius: number; index: number }> = [];
		for (let i = 0; i < saved.length; i++) {
			const g = saved[i];
			if (g && !Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)) {
				validWithIndices.push({
					lat: g.lat,
					lng: g.lng,
					radius: 0,
					index: i + 1
				});
			}
		}

		if (validWithIndices.length < 2) {
			goto(routes.toolsRange());
			return;
		}

		this.gcps = validWithIndices;
		this.radii = new Array(validWithIndices.length).fill(0);
	}

	setRadius = (index: number, val: number) => {
		const next = [...this.radii];
		next[index] = Math.max(0, val);
		this.radii = next;

		const nextGcps = this.gcps.map((g, i) => ({
			...g,
			radius: this.radii[i] || 0
		}));
		this.gcps = nextGcps;
	};

	mapGcpsWithIndex = $derived(
		this.gcps.map((g, i) => ({
			...g,
			radius: this.radii[i] || 0
		}))
	);

	suggestedLocation = $derived.by(() => {
		const operatorPos = get(locationStore);
		const inputGcps = this.gcps.map((g, i) => ({
			lat: g.lat,
			lng: g.lng,
			radius: this.radii[i] || 0
		}));
		return triangulateTree(inputGcps, operatorPos);
	});

	handleBack = () => {
		goto(routes.toolsRange());
	};

	handleAddTree = () => {
		if (this.suggestedLocation) {
			goto(routes.treeAdd());
		}
	};
}
