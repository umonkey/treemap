// Listen to map movements, load markers, put them in the store.

import { apiClient } from '$lib/api';
import { getMap } from '$lib/map';
import { markerStore } from '$lib/stores/markerStore';
import { toast } from '@zerodevx/svelte-toast';
import { type Map, type LatLngBounds } from 'leaflet';
import { type MountFn } from '$lib/types';
import { searchStore } from '$lib/stores';
import { get } from 'svelte/store';

// Expand current map bounds by 100% in all directions, one extra screen.
// This makes us load some extra markers, which makes panning more natural.
const expand = (bounds: LatLngBounds) => {
	const ns = bounds.getNorth() - bounds.getSouth();
	const ew = bounds.getEast() - bounds.getWest();

	return {
		n: bounds.getNorth() + ns,
		e: bounds.getEast() + ew,
		s: bounds.getSouth() - ns,
		w: bounds.getWest() - ew
	};
};

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	let map: Map;

	const reload = async () => {
		console.debug('[map] Reloading markers.');

		const bounds = map.getBounds();
		const { n, e, s, w } = expand(bounds);

		const { status, data, error } = await apiClient.getMarkers(
			n,
			e,
			s,
			w,
			get(searchStore) ?? null
		);

		if (status === 200 && data) {
			console.debug(`[map] Received ${data.trees.length} markers.`);
			markerStore.set(data.trees);
		} else if (error) {
			toast.push(`Error loading markers: ${error.description}`);
		}
	};

	const handleMove = () => {
		console.debug('[map] Map moved, reloading markers.');
		reload();
	};

	onMount(() => {
		map = getMap();
		map.on('moveend', handleMove);

		// Trigger the initial load of markers.
		reload();

		return () => {
			map.off('moveend', handleMove);
		};
	});
};
