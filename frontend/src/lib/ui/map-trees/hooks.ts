// Listen to marker cache update, render items.

import { getMap } from '$lib/map';
import { markerStore } from '$lib/stores/markerStore';
import L, { type LayerGroup, type Map } from 'leaflet';
import type { ITree, MountFn } from '$lib/types';
import { get, writable } from 'svelte/store';
import { mapBus } from '$lib/buses';

const getTreeCircleProps = (tree: ITree) => {
	// Default color is for healthy trees.
	const props = {
		radius: 0.5,
		fillColor: '#228B22',
		color: '#228B22',
		weight: 1,
		opacity: 1,
		fillOpacity: 0.5
	};

	props.radius = Math.max(0.5, (tree.diameter ?? 4) / 2);

	if (tree.state === 'stomp') {
		props.color = '#000';
		props.fillColor = '#000';
		props.fillOpacity = 0.2;
		props.radius = 1;
	} else if (tree.state === 'sick' || tree.state === 'deformed') {
		props.color = '#228B22';
		props.fillColor = '#FFD700';
	} else if (tree.state === 'dead') {
		props.color = '#8B4513';
		props.fillColor = '#8B4513';
		props.fillOpacity = 0.2;
	}

	if (!tree.diameter) {
		props.opacity = 0.25;
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
	let map: Map;

	// Previously rendered items, to remove on update.
	const previous = writable<LayerGroup | null>(null);

	// Last clicked tree id.
	const lastId = writable<string | null>(null);

	const handleClick = (tree: ITree) => {
		console.debug(`[map] Tree ${tree.id} clicked.`);

		mapBus.emit('select', tree.id);

		// Re-center on last clicked tree.  Scenario:
		// (1) Click a tree, map centers on it, triggered by the MapPreview component.
		// (2) Move the map manually.
		// (3) Click the same tree again, map should re-center.
		if (tree.id === get(lastId)) {
			mapBus.emit('center', {
				lat: tree.lat,
				lng: tree.lon
			});
		}

		lastId.set(tree.id);
	};

	const renderTrees = (trees: ITree[]): LayerGroup => {
		const group = L.layerGroup();

		// Draw crowns first.  They are bigger and overlap a lot,
		// so better keep them down.
		trees.forEach((tree: ITree) => {
			const point = L.circle([tree.lat, tree.lon], getTreeCircleProps(tree));
			point.on('click', () => handleClick(tree));
			point.addTo(group);
		});

		// Draw trunks on top of crowns.
		trees.forEach((tree: ITree) => {
			const point = L.circle([tree.lat, tree.lon], getTrunkProps(tree));
			point.on('click', () => handleClick(tree));
			point.addTo(group);
		});

		return group;
	};

	const handleChange = (trees: ITree[]) => {
		const layer = renderTrees(trees);

		get(previous)?.remove();
		previous.set(layer);

		layer.addTo(map);
	};

	onMount(() => {
		map = getMap();
		markerStore.subscribe(handleChange);
	});
};
