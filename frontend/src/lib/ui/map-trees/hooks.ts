// Listen to marker cache update, render items.

import { mapBus } from '$lib/buses';
import { getMap } from '$lib/map';
import { markerStore } from '$lib/stores/markerStore';
import type { ITree, MountFn } from '$lib/types';
import L, { type LayerGroup, type Map } from 'leaflet';
import { get, writable } from 'svelte/store';

const getTreeCircleProps = (tree: ITree) => {
	// Default color is for healthy trees.
	const props = {
		radius: 0.5,
		fillColor: '#228B22',
		weight: 1,
		opacity: 0,
		fillOpacity: 0.5
	};

	props.radius = Math.max(0.5, (tree.diameter ?? 4) / 2);

	if (tree.state === 'stump' || tree.state === 'gone') {
		props.fillColor = '#000';
		props.fillOpacity = 0.2;
		props.radius = 0.5;
	} else if (tree.state === 'unknown') {
		props.fillColor = '#FFD700';
	} else if (tree.state === 'dead') {
		props.fillColor = '#8B4513';
		props.fillOpacity = 0.2;
	}

	if (!tree.diameter) {
		props.fillOpacity = 0.25;
	}

	return props;
};

const getTrunkProps = (tree: ITree) => {
	const props = {
		radius: 0.05,
		fillColor: '#000',
		color: '#000',
		weight: 1,
		opacity: 1,
		fillOpacity: 0.8
	};

	if (tree.circumference) {
		props.radius = Math.max(0.05, tree.circumference / 2 / Math.PI);
	} else {
		props.opacity = 0.25;
		props.fillOpacity = 0.25;
	}

	return props;
};

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	const map = writable<Map | null>(null);

	// Previously rendered items, to remove on update.
	const previous = writable<LayerGroup | null>(null);

	// Last clicked tree id.
	const lastId = writable<string | null>(null);

	const handleClick = (tree: ITree) => {
		console.debug(`[map] Tree ${tree.id} clicked.`);

		mapBus.emit('select', tree.id);

		if (navigator.vibrate) {
			navigator.vibrate(50);
		}

		// Re-center on last clicked tree.  Scenario:
		// (1) Click a tree, map centers on it, triggered by the MapPreview component.
		// (2) Move the map manually.
		// (3) Click the same tree again, map should re-center.
		if (tree.id === get(lastId)) {
			mapBus.emit('pin', {
				lat: tree.lat,
				lng: tree.lon
			});
		}

		lastId.set(tree.id);
	};

	const handleContextMenu = (tree: ITree, e: L.LeafletMouseEvent) => {
		L.DomEvent.stop(e);

		console.debug(`[map] Tree ${tree.id} context menu.`);

		mapBus.emit('menu', tree.id);

		if (navigator.vibrate) {
			navigator.vibrate([50, 50]);
		}
	};

	const renderTrees = (trees: ITree[]): LayerGroup => {
		const group = L.layerGroup();

		// Draw crowns first.  They are bigger and overlap a lot,
		// so better keep them down.
		for (const tree of trees) {
			const point = L.circle([tree.lat, tree.lon], getTreeCircleProps(tree));
			point.on('click', () => handleClick(tree));
			point.on('contextmenu', (e: L.LeafletMouseEvent) => handleContextMenu(tree, e));
			point.addTo(group);
		}

		// Draw trunks on top of crowns.
		for (const tree of trees) {
			const point = L.circle([tree.lat, tree.lon], getTrunkProps(tree));
			point.on('click', () => handleClick(tree));
			point.on('contextmenu', (e: L.LeafletMouseEvent) => handleContextMenu(tree, e));
			point.addTo(group);
		}

		return group;
	};

	const handleChange = (trees: ITree[]) => {
		const m = get(map);

		if (m === null) {
			console.debug('[map] Map is not initialized, skipping rendering trees.');
			return;
		}

		console.debug(`[map] Rendering ${trees.length} trees.`);

		const layer = renderTrees(trees);

		get(previous)?.remove();
		previous.set(layer);

		try {
			layer.addTo(m);
		} catch (e) {
			console.error('[map] Error adding layer to map:', e);
		}
	};

	onMount(() => {
		map.set(getMap());
		markerStore.subscribe(handleChange);

		return () => {
			console.debug('[map] Unsubscribing from markerStore.');
			map.set(null);
		};
	});
};
