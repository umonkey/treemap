import { routes, goto } from '$lib/routes';
import { locationStore } from '$lib/stores/locationStore';
import { get } from 'svelte/store';
import { rangeStore } from '../store.svelte';
import { triangulateTree } from '$lib/utils/triangulation';
import type { IGcpWithRadius } from './MapPreview.svelte.ts';

export class RangeEnterState {
	gcps = $state<IGcpWithRadius[]>([]);
	radii = $state<number[]>([]);

	constructor() {
		const validGcps = rangeStore.validGcps;
		if (validGcps.length < 2) {
			goto(routes.toolsRange());
			return;
		}

		const validWithIndices: IGcpWithRadius[] = [];
		for (let i = 0; i < rangeStore.gcps.length; i++) {
			const g = rangeStore.gcps[i];
			if (g && !Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)) {
				validWithIndices.push({
					lat: g.lat,
					lng: g.lng,
					radius: 0,
					index: i + 1
				});
			}
		}

		this.gcps = validWithIndices;
		this.radii = new Array(validWithIndices.length).fill(0);
	}

	setRadius = (index: number, val: number) => {
		const next = [...this.radii];
		next[index] = Math.max(0, val);
		this.radii = next;
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

	trees = $derived(rangeStore.trees);

	removeTree = (id: string) => {
		rangeStore.removeTree(id);
	};

	handleBack = () => {
		goto(routes.toolsRange());
	};

	handleAddTree = () => {
		if (this.suggestedLocation) {
			rangeStore.addTree(this.suggestedLocation);
			this.radii = new Array(this.gcps.length).fill(0);
		}
	};
}
