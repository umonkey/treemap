// Wrap the very low-level logic of Leaflet markers.
//
// TODO:
// - Change marker size based on zoom level, https://gis.stackexchange.com/questions/216558/leaflet-resize-markers-in-layer-when-zoom-in

import { apiClient } from '$lib/api';
import type { ITree } from '$lib/types';
import L from 'leaflet';
import type { LatLngBounds, Map } from 'leaflet';
import { mapBus } from '$lib/buses';
import { mapLastTree } from '$lib/stores/mapStore';
import { get } from 'svelte/store';
import { markerStore } from '$lib/stores/markerStore';

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

export class Markers {
	private map;

	private searchQuery: string | undefined = undefined;
	private bounds: LatLngBounds | undefined = undefined;

	private oldMarkers: L.Layer[] = [];

	constructor(map: Map, searchQuery: string | undefined) {
		this.map = map;
		this.searchQuery = searchQuery;

		map.on('moveend', () => this.onMoveEnd());

		// Initiate the first load.
		this.onMoveEnd();

		// While we're loading new markers, show the ones from the previous time.
		this.replaceMarkers(get(markerStore));
	}

	public setSearchQuery(query: string | undefined) {
		if ((this.searchQuery ?? null) !== (query ?? null)) {
			this.searchQuery = query;
			this.reload();
		}
	}

	private async onMoveEnd() {
		console.debug('[map] Map moved, reloading markers.');
		this.bounds = this.map.getBounds();
		this.reload();
	}

	/**
	 * Reload markers after a change in parameters.
	 */
	private reload() {
		const query = this.searchQuery ?? null;

		if (!this.bounds) {
			console.debug('[map] Not reloading -- bounds not set.');
			return;
		}

		console.debug(`[map] Reloading markers, search=${query}.`);

		const { n, e, s, w } = expand(this.bounds);

		apiClient.getMarkers(n, e, s, w, query).then((res) => {
			// This is a hot fix for how the markers are added.
			//
			// We first initialize the map, and add the marker loader (this class),
			// then the effect triggers which adds the search query.  This makes us send
			// two simultaneous requests, which one comes first we don't know.
			//
			// The right solution would be to refactor the map hooks, to make sure we don't
			// start loading markers before we know the right search query.
			const current = this.searchQuery ?? null;

			if (query !== current) {
				console.debug(`[map] Search query overruled; received=${query}, current=${current}.`);
				return;
			}

			if (res.status === 200 && res.data) {
				const trees = res.data.trees;
				console.debug(`[map] Received ${trees.length} trees, search=${query}.`);
				markerStore.set(trees);
				this.replaceMarkers(trees);
			}
		});
	}

	/**
	 * Replaces the current markers with the given markers.
	 *
	 * Leaflet cannot track duplicates so we have to do this on our side.
	 */
	private replaceMarkers(trees: ITree[]) {
		const items = this.getItemsToShow(trees);

		// Remove old items.
		for (const item of this.oldMarkers) {
			this.map.removeLayer(item);
		}

		// Add new items.
		for (const item of items) {
			this.map.addLayer(item);
			this.oldMarkers.push(item);
		}
	}

	// Add trees as individual markers.
	//
	// Trunks are added on top of crowns, to make them clickable.
	private getItemsToShow(trees: ITree[]): L.Layer[] {
		const crowns = [];
		const trunks = [];

		for (const tree of trees) {
			const point = L.circle([tree.lat, tree.lon], this.getTreeCircleProps(tree));
			const trunk = L.circle([tree.lat, tree.lon], this.getTrunkProps(tree));

			const clickHandler = () => {
				mapBus.emit('select', tree.id);

				// Force map center change.
				//
				// This is also happening in select above, when the tree
				// is changed, but in case you click a tree, then move the
				// map, then click the same tree again, the id won't change
				// and the component won't move the map as the center change
				// effect didn't fire.  This is a double check.
				//
				// In the map component we make sure not to do any extra work
				// if we're asked to center on the point where we are already.
				if (tree.id === get(mapLastTree)) {
					mapBus.emit('center', {
						lat: tree.lat,
						lng: tree.lon
					});
				}
			};

			point.on('click', clickHandler);
			trunk.on('click', clickHandler);

			crowns.push(point);
			trunks.push(trunk);
		}

		return [...crowns, ...trunks];
	}

	// Returns props for a tree circle, used in the map component.
	private getTreeCircleProps(tree: ITree) {
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

		// console.debug(`[map] Tree ${tree.id} crown=${props.radius}`);

		return props;
	}

	// Returns props for a tree circle, used in the map component.
	private getTrunkProps(tree: ITree) {
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

		// console.debug(`[map] Tree ${tree.id} trunk=${props.radius}`);

		return props;
	}
}
